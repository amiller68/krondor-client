// @ts-ignore
import { ethers } from 'hardhat';

/* Blog Deployment script */
async function main() {
  // Get the contract factory and signer
  const gas = await ethers.provider.getGasPrice();
  const BlogContract = await ethers.getContractFactory('Blog');
  // Deploy the contract
  console.log('Deploying blog contract...');
  const blog = await BlogContract.deploy();
  await blog.deployed();
  console.log('Blog deployed to:', blog.address);
  console.log('Gas price:', gas.toString());
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
