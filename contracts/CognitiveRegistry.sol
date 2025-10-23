// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract CognitiveRegistry is Ownable, ReentrancyGuard {
    
    struct CognitiveNode {
        address validator;
        string nodeId;
        uint256 stakedAmount;
        uint256 reputationScore;
        uint256 tasksCompleted;
        bool isActive;
        uint256 registeredAt;
    }
    
    struct ReasoningTask {
        uint256 taskId;
        string taskType; // "market_prediction", "anomaly_detection", etc
        bytes32 dataHash;
        address requester;
        uint256 reward;
        uint256 deadline;
        bool completed;
        address assignedNode;
    }
    
    mapping(address => CognitiveNode) public cognitiveNodes;
    mapping(uint256 => ReasoningTask) public reasoningTasks;
    mapping(address => bool) public isRegistered;
    
    address[] public nodeList;
    uint256 public taskCounter;
    uint256 public minStakeAmount = 0.1 ether; // 0.1 MATIC (cheap for testnet)
    
    event NodeRegistered(address indexed validator, string nodeId);
    event NodeDeactivated(address indexed validator);
    event TaskCreated(uint256 indexed taskId, string taskType, uint256 reward);
    event TaskAssigned(uint256 indexed taskId, address indexed node);
    event TaskCompleted(uint256 indexed taskId, address indexed node, uint256 reward);
    event ReputationUpdated(address indexed node, uint256 newScore);
    
    constructor() Ownable(msg.sender) {}
    
    function registerCognitiveNode(string memory nodeId) external payable {
        require(!isRegistered[msg.sender], "Already registered");
        require(msg.value >= minStakeAmount, "Insufficient stake");
        
        cognitiveNodes[msg.sender] = CognitiveNode({
            validator: msg.sender,
            nodeId: nodeId,
            stakedAmount: msg.value,
            reputationScore: 100,
            tasksCompleted: 0,
            isActive: true,
            registeredAt: block.timestamp
        });
        
        isRegistered[msg.sender] = true;
        nodeList.push(msg.sender);
        
        emit NodeRegistered(msg.sender, nodeId);
    }
    
    function createReasoningTask(
        string memory taskType,
        bytes32 dataHash,
        uint256 deadline
    ) external payable returns (uint256) {
        require(msg.value > 0, "Reward required");
        
        taskCounter++;
        reasoningTasks[taskCounter] = ReasoningTask({
            taskId: taskCounter,
            taskType: taskType,
            dataHash: dataHash,
            requester: msg.sender,
            reward: msg.value,
            deadline: deadline,
            completed: false,
            assignedNode: address(0)
        });
        
        emit TaskCreated(taskCounter, taskType, msg.value);
        return taskCounter;
    }
    
    function assignTask(uint256 taskId) external {
        require(isRegistered[msg.sender], "Not registered");
        require(cognitiveNodes[msg.sender].isActive, "Node not active");
        
        ReasoningTask storage task = reasoningTasks[taskId];
        require(!task.completed, "Task completed");
        require(task.assignedNode == address(0), "Task already assigned");
        require(block.timestamp < task.deadline, "Task expired");
        
        task.assignedNode = msg.sender;
        emit TaskAssigned(taskId, msg.sender);
    }
    
    function submitTaskResult(
        uint256 taskId,
        bytes32 resultHash
    ) external nonReentrant {
        ReasoningTask storage task = reasoningTasks[taskId];
        require(task.assignedNode == msg.sender, "Not assigned to you");
        require(!task.completed, "Already completed");
        require(block.timestamp < task.deadline, "Deadline passed");
        
        task.completed = true;
        
        CognitiveNode storage node = cognitiveNodes[msg.sender];
        node.tasksCompleted++;
        node.reputationScore += 10;
        
        uint256 reward = task.reward;
        (bool success, ) = msg.sender.call{value: reward}("");
        require(success, "Transfer failed");
        
        emit TaskCompleted(taskId, msg.sender, reward);
        emit ReputationUpdated(msg.sender, node.reputationScore);
    }
    
    function getActiveNodes() external view returns (address[] memory) {
        uint256 activeCount = 0;
        for (uint256 i = 0; i < nodeList.length; i++) {
            if (cognitiveNodes[nodeList[i]].isActive) {
                activeCount++;
            }
        }
        
        address[] memory activeNodes = new address[](activeCount);
        uint256 index = 0;
        for (uint256 i = 0; i < nodeList.length; i++) {
            if (cognitiveNodes[nodeList[i]].isActive) {
                activeNodes[index] = nodeList[i];
                index++;
            }
        }
        
        return activeNodes;
    }
    
    function deactivateNode() external {
        require(isRegistered[msg.sender], "Not registered");
        cognitiveNodes[msg.sender].isActive = false;
        emit NodeDeactivated(msg.sender);
    }
    
    receive() external payable {}
}
