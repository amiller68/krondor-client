// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

/// @title Backend
/// An upgrade
contract Backend {
  // File struct
  struct File {
    // Where it is stored on local disk
    string path;
    // When last updated
    uint256 timestamp;
    // The file's cid
    string cid;
    string metadata;
  }
  // An event to be emitted when a file is touched
  event FileTouched(string path, uint256 timestamp);
  // An event to be emitted when a file is deleted
  event FileDeleted(string path, uint256 timestamp);
  // We maintain a mapping of post ids to posts
  mapping(uint => File) posts;

  // We declare a contract owner
  address public owner;

  // We set the contract owner to the address that deployed the contract
  constructor() {
    owner = msg.sender;
    postCount = 0;
  }

  // Require that the caller is the contract owner
  modifier onlyOwner() {
    require(msg.sender == owner, 'Only the owner can call this function.');
    _;
  }

  /// File Management

  // Create a new file
  function touchFile(
    string memory _path,
    string memory _cid,
    string memory _metadata
    )
  public onlyOwner (uint) {
    require(bytes(_path).length > 0, 'Path is required.');
    require(bytes(_cid).length > 0, 'CID is required.');
    // Hash the path to get the file id
    uint fileId = uint(keccak256(abi.encodePacked(_path)));
    // If the file already exists, update it
    if (posts[fileId].timestamp > 0) {
      posts[fileId].cid = _cid;
      posts[fileId].metadata = _metadata;
      posts[fileId].timestamp = block.timestamp;
    } else {
      // Otherwise, create a new file
      posts[fileId] = File(_path, block.timestamp, _cid, _metadata);
    }
    emit FileTouched(_path, block.timestamp);
    return fileId;
  }


  /// Content Management
//
//  // Create a new post
//  function createPost(
//    string memory _title,
//    string memory _cid
//  ) public onlyOwner {
//    require(bytes(_title).length > 0, 'Title is required.');
//    require(bytes(_cid).length > 0, 'CID is required.');
//    postCount++;
//    usedCount++;
//    posts[usedCount] = Post(_title, block.timestamp, _cid);
//    emit NewPost(usedCount, block.timestamp);
//  }
//
//  // Delete a post by id
//  function deletePost(uint _postId) public onlyOwner (uint) {
//    delete posts[_postId];
//    postCount--;
//    emit DeletePost(_postId, block.timestamp);
//    return _postId;
//  }
//
//  // Update a post by id
//  function updatePost(
//    uint _postId,
//    string memory _title,
//    string memory _cid
//  ) public onlyOwner (uint) {
//    // If both the title and cid are empty, we don't need to update anything
//    require(
//      bytes(_title).length > 0 || bytes(_cid).length > 0,
//      'Title or CID is required.'
//    );
//
//    // If the title is not empty, update the title
//    if (bytes(_title).length > 0) {
//      posts[_postId].title = _title;
//    }
//    // If the cid is not empty, update the cid
//    if (bytes(_cid).length > 0) {
//      posts[_postId].cid = _cid;
//    }
//    // Update the timestamp
//    posts[_postId].timestamp = block.timestamp;
//    emit UpdatePost(_postId, block.timestamp);
//    return _postId;
//  }
//
//  /// Content Retrieval
//
//  // Get the total number of posts
//  function getPostCount() public view returns (uint) {
//    return postCount;
//  }
//
//  // Get post by id
//  function getPost(
//    uint _postId
//  ) public view returns (string memory, uint256, string memory) {
//    // If the timestamp is 0, revert
//    require(posts[_postId].timestamp > 0, 'Post does not exist.');
//    return (posts[_postId].title, posts[_postId].timestamp, posts[_postId].cid);
//  }
//
//  // Get all posts
//  function getAllPosts()
//    public
//    view
//    returns (string[] memory, uint256[] memory, string[] memory)
//  {
//    string[] memory titles = new string[](postCount);
//    uint256[] memory timestamps = new uint256[](postCount);
//    string[] memory cids = new string[](postCount);
//
//    uint index = 0;
//    for (uint i = 1; i <= usedCount; i++) {
//      if (posts[i].timestamp > 0) {
//        titles[index] = posts[i].title;
//        timestamps[index] = posts[i].timestamp;
//        cids[index] = posts[i].cid;
//        index++;
//      }
//    }
//    return (titles, timestamps, cids);
//  }
//
//  // Get all posts with pagination - skip deleted posts
//  function getPostsByPage(
//    uint _page,
//    uint _perPage
//  ) public view returns (string[] memory, uint256[] memory, string[] memory) {
//    uint start = _page * _perPage;
//    uint end = start + _perPage;
//    string[] memory titles = new string[](_perPage);
//    uint256[] memory timestamps = new uint256[](_perPage);
//    string[] memory cids = new string[](_perPage);
//    uint index = 0;
//    for (uint i = start + 1; i < end + 1; i++) {
//      if (posts[i].timestamp != 0) {
//        titles[index] = posts[i].title;
//        timestamps[index] = posts[i].timestamp;
//        cids[index] = posts[i].cid;
//        index++;
//      }
//    }
//    return (titles, timestamps, cids);
//  }cids
}
