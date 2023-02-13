import { time, loadFixture } from '@nomicfoundation/hardhat-network-helpers';
const { anyValue } = require('@nomicfoundation/hardhat-chai-matchers/withArgs');
import { expect } from 'chai';
// @ts-ignore
import { ethers } from 'hardhat';

describe('CrudFs', function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deployCrudFsFixture() {
    // Contracts are deployed using the first signer/account by default
    const [owner, otherAccount] = await ethers.getSigners();

    const CrudFs = await ethers.getContractFactory('CrudFs');
    const crudFs = await CrudFs.deploy();

    return { owner, otherAccount, crudFs };
  }

  // Test whether the crudFs is deployed correctly
  describe('Deployment', function () {
    it('Should set the right owner', async function () {
      const { owner, crudFs } = await loadFixture(deployCrudFsFixture);
      expect(await crudFs.owner()).to.equal(owner.address);
    });

    it("Shouldn't have any posts", async function () {
      const { crudFs } = await loadFixture(deployCrudFsFixture);
      expect(Number(await crudFs.readFileCount())).to.equal(0);
    });
    it('Should not be writable by other accounts', async function () {
      const { otherAccount, crudFs } = await loadFixture(deployCrudFsFixture);
      // Decalare a Path for the file
      const path = 'test.txt';
      // Decalre a CID for the file
      const cid = 'Qm12345';
      // Try to write a post
      await expect(
        crudFs.connect(otherAccount).createFile(path, cid, '')
        // @ts-ignore - chai-matchers doesn't get picked up, but this should work
      ).to.be.revertedWith('Ownable: caller is not the owner');
    });
  });

  // Test whether the crudFs is correct
  describe('CrudFs', function () {
    // Test Paths
    const path_0 = 'test0.txt';
    const path_1 = 'test1.txt';
    const path_2 = 'test2.txt';
    const path_3 = 'test3.txt';

    // Get the keccak256 hash of the paths
    const fileKey_0 = ethers.utils.keccak256(ethers.utils.toUtf8Bytes(path_0));
    const fileKey_1 = ethers.utils.keccak256(ethers.utils.toUtf8Bytes(path_1));
    const fileKey_2 = ethers.utils.keccak256(ethers.utils.toUtf8Bytes(path_2));
    const fileKey_3 = ethers.utils.keccak256(ethers.utils.toUtf8Bytes(path_3));

    // Test CIDs
    const cid_0 = 'Qm12345';
    const cid_1 = 'Qm67890';
    const cid_2 = 'Qm09876';
    const cid_3 = 'Qm54321';



    describe('Create', function () {
      it('Should create a file', async function () {
        const { owner, crudFs } = await loadFixture(deployCrudFsFixture);
        // Create a file and check that it was created correctly by reading the logs
        await expect(crudFs.createFile(path_0, cid_0, ''))
          // @ts-ignore - This is correct
          .to.emit(crudFs, 'CreateFile')
          // Check that the file was created correctly
          .withArgs(fileKey_0, anyValue, cid_0, '');
        // Check that the file was created correctly
        const file = await crudFs.readFile(fileKey_0);
        expect(file.path).to.equal(path_0);
        expect(file.cid).to.equal(cid_0);
        expect(file.metadata).to.equal('');
      });

      it('Should not create a file with an empty CID', async function () {
        const { owner, crudFs } = await loadFixture(deployCrudFsFixture);
        // Create a file and check that it was created correctly by reading the logs
        await expect(crudFs.createFile(path_0, '', ''))
          // @ts-ignore - This is correct
          .to.be.revertedWith('CID cannot be empty');
      });
    });

    describe('Read', function () {
      it('Should read a file', async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        // Create a file and check that it was created correctly by reading the logs
        await crudFs.createFile(path_0, cid_0, '{ "name": "test" }');
        // Check that the file was created correctly
        const file = await crudFs.readFile(fileKey_0);
        expect(file.path).to.equal(path_0);
        expect(file.cid).to.equal(cid_0);
        // Interpret the timestamp and check that it is from today
        const timestamp = Number(file.timestamp);
        const today = new Date();
        const timestampDate = new Date(timestamp * 1000);
        expect(timestampDate.getDate()).to.equal(today.getDate());
        expect(timestampDate.getMonth()).to.equal(today.getMonth());
        expect(timestampDate.getFullYear()).to.equal(today.getFullYear());
        // Interpret the Metadata and check that it is correct
        const metadataJson = JSON.parse(file.metadata);
        expect(metadataJson.name).to.equal('test');
      });

      it("Should not read a file that doesn't exist", async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        // Create a Bytes32 array of 0s for the fileKey
        const fileKey = ethers.utils.hexlify(ethers.utils.randomBytes(32));
        // Try to read a file that doesn't exist
        // @ts-ignore - This is correct
        await expect(crudFs.readFile(fileKey)).to.be.revertedWith(
          'File does not exist.'
        );
      });

      it('Should reutrn the correct number of files', async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        // Create a file and check that it was created correctly by reading the logs
        await crudFs.createFile(path_0, cid_0, '');
        // Check that the file was created correctly
        const fileCount = await crudFs.readFileCount();
        expect(Number(fileCount)).to.equal(1);
      });

      it('Should return the right file key for a given index', async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        // Create a file and check that it was created correctly by reading the logs
        await crudFs.createFile(path_0, cid_0, '');
        // Check that the file was created correctly
        const fileKey = await crudFs.readFileKeyAtIndex(0);
        expect(fileKey).to.equal(fileKey_0);
      });

      it('Should revert if the index is out of bounds', async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        // @ts-ignore - This is correct
        await expect(crudFs.readFileKeyAtIndex(1)).to.be.revertedWith(
          'Index out of bounds.'
        );
      });

      it('Should return the right file for a given index', async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        // Create a file and check that it was created correctly by reading the logs
        await crudFs.createFile(path_0, cid_0, '');
        // Check that the file was created correctly
        const file = await crudFs.readFileAtIndex(0);
        expect(file.cid).to.equal(cid_0);
      });

      it('Should revert if the index is out of bounds', async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        // @ts-ignore - This is correct
        await expect(crudFs.readFileAtIndex(1)).to.be.revertedWith(
          'Index out of bounds.'
        );
      });

      it('Should return the right file keys for the entire list', async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        // Create four files and check that they were created correctly by reading the keys
        await crudFs.createFile(path_0, cid_0, '');
        await crudFs.createFile(path_1, cid_1, '');
        await crudFs.createFile(path_2, cid_2, '');
        await crudFs.createFile(path_3, cid_3, '');
        // Check that the file was created correctly
        const fileKeys = await crudFs.readAllFileKeys();
        expect(fileKeys[0]).to.equal(fileKey_0);
        expect(fileKeys[1]).to.equal(fileKey_1);
        expect(fileKeys[2]).to.equal(fileKey_2);
        expect(fileKeys[3]).to.equal(fileKey_3);
      });

      it('Should return the right files for the entire list', async function () {
        // Create four files and check that they were created by reading them by a list of keys
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        await crudFs.createFile(path_0, cid_0, '');
        await crudFs.createFile(path_1, cid_1, '');
        await crudFs.createFile(path_2, cid_2, '');
        await crudFs.createFile(path_3, cid_3, '');
        // Check that the file was created correctly
        // Results are returned as a tupe of lists
        const files = await crudFs.readFiles([
          fileKey_0,
          fileKey_1,
          fileKey_2,
          fileKey_3,
        ]);
        // Check that the path is correct
        expect(files[0][0]).to.equal(path_0);
        expect(files[0][1]).to.equal(path_1);
        expect(files[0][2]).to.equal(path_2);
        expect(files[0][3]).to.equal(path_3);
        // Check that the cid is correct
        expect(files[1][0]).to.equal(cid_0);
        expect(files[1][1]).to.equal(cid_1);
        expect(files[1][2]).to.equal(cid_2);
        expect(files[1][3]).to.equal(cid_3);
        // Check that timestamp is correct -- not 0
        expect(files[2][0]).to.not.equal(0);
        expect(files[2][1]).to.not.equal(0);
        expect(files[2][2]).to.not.equal(0);
        expect(files[2][3]).to.not.equal(0);
        // Check that the data is correct
        expect(files[3][0]).to.equal('');
        expect(files[3][1]).to.equal('');
        expect(files[3][2]).to.equal('');
        expect(files[3][3]).to.equal('');
      });

      it('Should revert if a key doesnt exist in the list', async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        await crudFs.createFile(path_0, cid_0, '');
        await crudFs.createFile(path_1, cid_1, '');
        await crudFs.createFile(path_2, cid_2, '');
        await expect(
          crudFs.readFiles([fileKey_0, fileKey_1, fileKey_2, fileKey_3])
        )
          // @ts-ignore - This is correct
          .to.be.revertedWith('File does not exist.');
      });

      it('Should return all the files', async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        await crudFs.createFile(path_0, cid_0, 'zero');
        await crudFs.createFile(path_1, cid_1, 'one');
        await crudFs.createFile(path_2, cid_2, 'two');
        await crudFs.createFile(path_3, cid_3, 'three');
        // Read files with keys and metadata
        const files = await crudFs.readAllFiles();
        // Results are returned as a tupe of lists of keys, cids, timestamps, and metadata
        // Check the paths
        expect(files[0][0]).to.equal(path_0);
        expect(files[0][1]).to.equal(path_1);
        expect(files[0][2]).to.equal(path_2);
        expect(files[0][3]).to.equal(path_3);
        // Check the cids
        expect(files[1][0]).to.equal(cid_0);
        expect(files[1][1]).to.equal(cid_1);
        expect(files[1][2]).to.equal(cid_2);
        expect(files[1][3]).to.equal(cid_3);
        // Check the timestamps -- that they are not 0
        expect(files[2][0]).to.not.equal(0);
        expect(files[2][1]).to.not.equal(0);
        expect(files[2][2]).to.not.equal(0);
        expect(files[2][3]).to.not.equal(0);
        // Check the metadata
        expect(files[3][0]).to.equal('zero');
        expect(files[3][1]).to.equal('one');
        expect(files[3][2]).to.equal('two');
        expect(files[3][3]).to.equal('three');
      });
    });

    describe('Update', function () {
      it("Should revert if the file doesn't exist", async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        await expect(crudFs.updateFile(fileKey_0, cid_0, ''))
          // @ts-ignore - This is correct
          .to.be.revertedWith('File does not exist.');
      });
      it('Should update the file', async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        await crudFs.createFile(path_0, cid_0, '');
        await expect(crudFs.updateFile(fileKey_0, cid_1, 'hello'))
            // @ts-ignore - This is correct
            .to.emit(crudFs, 'UpdateFile')
            .withArgs(fileKey_0, anyValue, cid_1, 'hello');

        // Check that the file was updated correctly

        const file = await crudFs.readFile(fileKey_0);
        expect(file[0]).to.equal(path_0);
        expect(file[1]).to.equal(cid_1);
        expect(file[2]).to.not.equal(0);
        expect(file[3]).to.equal('hello');
      });
    });

    describe('Delete', function () {
      it("Should revert if the file doesn't exist", async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        await expect(crudFs.deleteFile(fileKey_0))
          // @ts-ignore - This is correct
          .to.be.revertedWith('File does not exist.');
      });
      it('Should delete the file', async function () {
        const { crudFs } = await loadFixture(deployCrudFsFixture);
        await crudFs.createFile(path_0, cid_0, '');
        await crudFs.deleteFile(fileKey_0);
        await expect(crudFs.readFile(fileKey_0))
          // @ts-ignore - This is correct
          .to.be.revertedWith('File does not exist.');
      });
    });
  });
});
