const hre = require("hardhat");

async function main() {
    console.log("🚀 Deploying PolyNeurons contracts...");

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
        const fundAmount = hre.ethers.parseEther("0.1"); // Use 0.1 MATIC for testnet
        const fundTx = await por.fundContract({ value: fundAmount });
        await fundTx.wait();
        console.log("💰 PoR contract funded with 0.1 MATIC");
    } catch (error) {
        console.log("⚠️  Skipping funding (insufficient balance). You can fund later with:");
        console.log(`   cast send ${porAddress} "fundContract()" --value 0.1ether --private-key $PRIVATE_KEY --rpc-url $AMOY_RPC_URL`);
    }

    console.log("\n📝 Update your .env file:");
    console.log(`REGISTRY_ADDRESS=${registryAddress}`);
    console.log(`POR_CONTRACT_ADDRESS=${porAddress}`);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
