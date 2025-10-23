const hre = require("hardhat");

async function main() {
    console.log("🚀 Deploying PolyNeurons contracts (Testnet Version)...");
    console.log("💡 Using cheaper stake amount: 0.1 MATIC");
    console.log("");

    // Deploy CognitiveRegistry
    const CognitiveRegistry = await hre.ethers.getContractFactory("CognitiveRegistry");
    const registry = await CognitiveRegistry.deploy();
    await registry.waitForDeployment();
    const registryAddress = await registry.getAddress();

    console.log("✅ CognitiveRegistry deployed to:", registryAddress);

    // Deploy ProofOfReasoning
    const ProofOfReasoning = await hre.ethers.getContractFactory("ProofOfReasoning");
    const por = await ProofOfReasoning.deploy();
    await por.waitForDeployment();
    const porAddress = await por.getAddress();

    console.log("✅ ProofOfReasoning deployed to:", porAddress);

    // Fund PoR contract (optional - can be done later)
    try {
        const fundAmount = hre.ethers.parseEther("0.05"); // Use 0.05 MATIC for testnet
        const fundTx = await por.fundContract({ value: fundAmount });
        await fundTx.wait();
        console.log("💰 PoR contract funded with 0.05 MATIC");
    } catch (error) {
        console.log("⚠️  Skipping funding (insufficient balance)");
        console.log(`   You can fund later with: cast send ${porAddress} "fundContract()" --value 0.05ether`);
    }

    console.log("");
    console.log("📝 Update your .env file:");
    console.log(`REGISTRY_ADDRESS=${registryAddress}`);
    console.log(`POR_CONTRACT_ADDRESS=${porAddress}`);
    console.log("");
    console.log("💡 Testnet Configuration:");
    console.log("   Min Stake: 0.1 MATIC");
    console.log("   Task Reward: 0.001 MATIC (recommended)");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
