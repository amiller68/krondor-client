// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import './unorderedKeySetLib.sol';
import '@openzeppelin/contracts/access/Ownable.sol';

/// @title CrudFs Contract
/// @author Alex Miller
/// @notice This contract is a simple file system for storing files on IPFS.
/// @dev This contract is not tested or audited. Do not use for production.

// SPECULATIVE LIFTS:
// TODO: Store CIDs in a more efficient way, maybe bytes32?
// TODO: Add a requirement that checks if CIDs are valid IPFS CIDs, replace isEmpty checks
// TODO: Store a CID to a JSON file that contains the metadata for the file, to save space
// TODO: Store a CID that references a JSON file or SQL database that contains all data in the contract

contract CrudFs is Ownable {
  // Use the UnorderedKeySetLib library for managing our file objects
  using UnorderedKeySetLib for UnorderedKeySetLib.Set;
  UnorderedKeySetLib.Set fileSet;

  /// A File struct that'd designed to be indexed by the hash of the file's path
  struct FileStruct {
    // The file's path on the file system
    string path;
    // The file's CID
    string cid;
    // When last updated
    uint256 timestamp;
    // The file's metadata -- this is a JSON string. Be responsible with the size of this.
    string metadata;
  }

  // A mapping that maps a hash of a file's path to a FileStruct
  mapping(bytes32 => FileStruct) files;

  // Events (Not sure if I want to emit Metadata yet)
  event CreateFile(
    bytes32 indexed key,
    uint256 indexed timestamp,
    string cid,
    string metadata
  );
  event UpdateFile(
    bytes32 indexed key,
    uint256 indexed timestamp,
    string cid,
    string metadata
  );
  event DeleteFile(bytes32 key);

  /// Public C.R.U.D. Functions

  // C is for 'Create'

  // Create a new file
  function createFile(
    string memory path,
    string memory cid,
    string memory metadata
  ) public onlyOwner {
    // Require that the path and cid are not empty
    require(bytes(path).length > 0, 'Path cannot be empty');
    require(bytes(cid).length > 0, 'CID cannot be empty');

    // Hash the path to get the key, revert if the file already exists
    bytes32 key = keccak256(abi.encodePacked(path));
    require(!fileSet.exists(key), 'File already exists.');

    // Insert the key into the fileSet
    fileSet.insert(key);
    FileStruct storage f = files[key];
    f.path = path;
    f.cid = cid;
    f.timestamp = block.timestamp;
    f.metadata = metadata;

    // Emit an event
    emit CreateFile(key, block.timestamp, cid, metadata);
  }

  // R is for 'Read'

  // Read a file by its key
  function readFile(
    bytes32 key
  )
    public
    view
    returns (
      string memory path,
      string memory cid,
      uint256 timestamp,
      string memory metadata
    )
  {
    // Revert if the file doesn't exist
    require(fileSet.exists(key), 'File does not exist.');
    FileStruct storage f = files[key];
    return (f.path, f.cid, f.timestamp, f.metadata);
  }

  // Read Multiple Files by their keys
  function readFiles(
    bytes32[] memory keys
  )
    public
    view
    returns (
      string[] memory paths,
      string[] memory cids,
      uint256[] memory timestamps,
      string[] memory metadata
    )
  {
    uint256 count = keys.length;
    // Initialize arrays for the struct members
    paths = new string[](count);
    cids = new string[](count);
    timestamps = new uint256[](count);
    metadata = new string[](count);
    // Loop through the keys and populate the arrays
    for (uint256 i = 0; i < count; i++) {
      // Revert if the file doesn't exist
      require(fileSet.exists(keys[i]), 'File does not exist.');
      FileStruct storage f = files[keys[i]];
      paths[i] = f.path;
      cids[i] = f.cid;
      timestamps[i] = f.timestamp;
      metadata[i] = f.metadata;
    }
    return (paths, cids, timestamps, metadata);
  }

  // Read the count of files
  function readFileCount() public view returns (uint256 count) {
    return fileSet.count();
  }

  // Read a file key at an index
  function readFileKeyAtIndex(uint256 index) public view returns (bytes32 key) {
    require(index < fileSet.count(), 'Index out of bounds.');
    return fileSet.keyAtIndex(index);
  }

  // Read a file by its index
  function readFileAtIndex(
    uint256 index
  )
    public
    view
    returns (
      string memory path,
      string memory cid,
      uint256 timestamp,
      string memory metadata
    )
  {
    require(index < fileSet.count(), 'Index out of bounds.');
    bytes32 key = fileSet.keyAtIndex(index);
    FileStruct storage f = files[key];
    return (f.path, f.cid, f.timestamp, f.metadata);
  }

  // Read all file keys in an array
  function readAllFileKeys() public view returns (bytes32[] memory keys) {
    uint256 count = fileSet.count();
    keys = new bytes32[](count);
    for (uint256 i = 0; i < count; i++) {
      keys[i] = fileSet.keyAtIndex(i);
    }
    return keys;
  }

  // Read all files. Return struct members as a tuple of arrays. Optional request to include keys and metadata.
  function readAllFiles()
    public
    view
    returns (
      string[] memory paths,
      string[] memory cids,
      uint256[] memory timestamps,
      string[] memory metadata
    )
  {
    uint256 count = fileSet.count();
    // Initialize arrays for the struct members
    paths = new string[](count);
    cids = new string[](count);
    timestamps = new uint256[](count);
    metadata = new string[](count);
    // Loop through the keys and populate the arrays
    for (uint256 i = 0; i < count; i++) {
      bytes32 key = fileSet.keyAtIndex(i);
      FileStruct storage f = files[key];
      paths[i] = f.path;
      cids[i] = f.cid;
      timestamps[i] = f.timestamp;
      metadata[i] = f.metadata;
    }
    return (paths, cids, timestamps, metadata);
  }

  // U is for 'Update'

  // Update a file
  function updateFile(
    bytes32 key,
    string memory cid,
    string memory metadata
  ) public onlyOwner {
    // Revert if the file doesn't exist
    require(fileSet.exists(key), 'File does not exist.');

    FileStruct storage f = files[key];
    f.cid = cid;
    f.timestamp = block.timestamp;
    f.metadata = metadata;

    // Emit an event
    emit UpdateFile(key, block.timestamp, cid, metadata);
  }

  // D is for 'Delete'

  // Delete a file
  function deleteFile(bytes32 key) public onlyOwner {
    require(fileSet.exists(key), 'File does not exist.');
    fileSet.remove(key);
    delete files[key];
    emit DeleteFile(key);
  }
}
