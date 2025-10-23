const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("CognitiveRegistry", function () {
    let registry;
    let owner, node1, node2, requester;

    beforeEach(async function () {
        [owner, node1, node2, requester] = await ethers.getSigners();

        const CognitiveRegistry = await ethers.getContractFactory("CognitiveRegistry");
        registry = await CognitiveRegistry.deploy();
        await registry.waitForDeployment();
    });

    describe("Node Registration", function () {
        it("Should register a cognitive node with stake", async function () {
            const stakeAmount = ethers.parseEther("100");

            await expect(
                registry.connect(node1).registerCognitiveNode("node-1", { value: stakeAmount })
            ).to.emit(registry, "NodeRegistered")
                .withArgs(node1.address, "node-1");

            const nodeData = await registry.cognitiveNodes(node1.address);
            expect(nodeData.isActive).to.be.true;
            expect(nodeData.stakedAmount).to.equal(stakeAmount);
        });

        it("Should reject registration with insufficient stake", async function () {
            const lowStake = ethers.parseEther("50");

            await expect(
                registry.connect(node1).registerCognitiveNode("node-1", { value: lowStake })
            ).to.be.revertedWith("Insufficient stake");
        });
    });

    describe("Task Management", function () {
        beforeEach(async function () {
            const stakeAmount = ethers.parseEther("100");
            await registry.connect(node1).registerCognitiveNode("node-1", { value: stakeAmount });
        });

        it("Should create a reasoning task", async function () {
            const reward = ethers.parseEther("1");
            const dataHash = ethers.keccak256(ethers.toUtf8Bytes("test data"));
            const deadline = Math.floor(Date.now() / 1000) + 3600;

            await expect(
                registry.connect(requester).createReasoningTask(
                    "market_prediction",
                    dataHash,
                    deadline,
                    { value: reward }
                )
            ).to.emit(registry, "TaskCreated");

            const task = await registry.reasoningTasks(1);
            expect(task.taskType).to.equal("market_prediction");
            expect(task.reward).to.equal(reward);
        });

        it("Should assign task to node", async function () {
            const reward = ethers.parseEther("1");
            const dataHash = ethers.keccak256(ethers.toUtf8Bytes("test data"));
            const deadline = Math.floor(Date.now() / 1000) + 3600;

            await registry.connect(requester).createReasoningTask(
                "market_prediction",
                dataHash,
                deadline,
                { value: reward }
            );

            await expect(
                registry.connect(node1).assignTask(1)
            ).to.emit(registry, "TaskAssigned")
                .withArgs(1, node1.address);
        });

        it("Should complete task and distribute reward", async function () {
            const reward = ethers.parseEther("1");
            const dataHash = ethers.keccak256(ethers.toUtf8Bytes("test data"));
            const deadline = Math.floor(Date.now() / 1000) + 3600;

            await registry.connect(requester).createReasoningTask(
                "market_prediction",
                dataHash,
                deadline,
                { value: reward }
            );

            await registry.connect(node1).assignTask(1);

            const resultHash = ethers.keccak256(ethers.toUtf8Bytes("result"));
            const balanceBefore = await ethers.provider.getBalance(node1.address);

            await registry.connect(node1).submitTaskResult(1, resultHash);

            const balanceAfter = await ethers.provider.getBalance(node1.address);
            expect(balanceAfter).to.be.gt(balanceBefore);

            const nodeData = await registry.cognitiveNodes(node1.address);
            expect(nodeData.tasksCompleted).to.equal(1);
        });
    });
});
