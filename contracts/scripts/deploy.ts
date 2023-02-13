// @ts-ignore
import { ethers } from 'hardhat';

/* Blog Deployment script */
async function main() {
  // Get the contract factory and signer
  const gas = await ethers.provider.getGasPrice();
  const CrudFs = await ethers.getContractFactory('CrudFs');
  // Deploy the contract
  console.log('Deploying blog contract...');
  const crudFs = await CrudFs.deploy();
  await crudFs.deployed();
  console.log('Blog deployed to:', crudFs.address);
  console.log('Gas price:', gas.toString());
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
