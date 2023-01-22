// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

/// @title Blog
/// An upgrade
contract Blog {
  // Our post is made up of blogs
  struct Post {
    string title;
    uint256 timestamp;
    string cid;
  }
  // An event to be emitted when a new post is created. It should index the postId and timestamp
  event NewPost(uint256 postId, uint256 timestamp);
  // We keep a public count of the number of posts
  uint public postCount;
  // We keep an internal count of how many spots in the mapping are used
  uint internal usedCount;
  // We maintain a mapping of post ids to posts
  mapping(uint => Post) posts;

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

  /// Content Management

  // Create a new post
  function createPost(
    string memory _title,
    string memory _cid
  ) public onlyOwner {
    require(bytes(_title).length > 0, 'Title is required.');
    require(bytes(_cid).length > 0, 'CID is required.');
    postCount++;
    usedCount++;
    posts[usedCount] = Post(_title, block.timestamp, _cid);
    emit NewPost(usedCount, block.timestamp);
  }

  // Delete a post by id
  function deletePost(uint _postId) public onlyOwner {
    delete posts[_postId];
    postCount--;
  }

  // Update a post by id
  function updatePost(
    uint _postId,
    string memory _title,
    string memory _cid
  ) public onlyOwner {
    // If both the title and cid are empty, we don't need to update anything
    require(
      bytes(_title).length > 0 || bytes(_cid).length > 0,
      'Title or CID is required.'
    );
    // If the title is not empty, update the title
    if (bytes(_title).length > 0) {
      posts[_postId].title = _title;
    }
    // If the cid is not empty, update the cid
    if (bytes(_cid).length > 0) {
      posts[_postId].cid = _cid;
    }
    // Update the timestamp
    posts[_postId].timestamp = block.timestamp;
  }

  /// Content Retrieval

  // Get the total number of posts
  function getPostCount() public view returns (uint) {
    return postCount;
  }

  // Get post by id
  function getPost(
    uint _postId
  ) public view returns (string memory, uint256, string memory) {
    // If the timestamp is 0, revert
    require(posts[_postId].timestamp > 0, 'Post does not exist.');
    return (posts[_postId].title, posts[_postId].timestamp, posts[_postId].cid);
  }

  // Get all posts
  function getAllPosts() public view returns (string[] memory, uint256[] memory, string[] memory) {
    string[] memory titles = new string[](postCount);
    uint256[] memory timestamps = new uint256[](postCount);
    string[] memory cids = new string[](postCount);

    uint index = 0;
    for (uint i = 1; i <= usedCount; i++) {
      if (posts[i].timestamp > 0) {
        titles[index] = posts[i].title;
        timestamps[index] = posts[i].timestamp;
        cids[index] = posts[i].cid;
        index++;
      }
    }
    return (titles, timestamps, cids);
  }

  // Get all posts with pagination - skip deleted posts
  function getPostsByPage(
    uint _page,
    uint _perPage
  ) public view returns (string[] memory, uint256[] memory, string[] memory) {
    uint start = _page * _perPage;
    uint end = start + _perPage;
    string[] memory titles = new string[](_perPage);
    uint256[] memory timestamps = new uint256[](_perPage);
    string[] memory cids = new string[](_perPage);
    uint index = 0;
    for (uint i = start + 1; i < end + 1; i++) {
      if (posts[i].timestamp != 0) {
        titles[index] = posts[i].title;
        timestamps[index] = posts[i].timestamp;
        cids[index] = posts[i].cid;
        index++;
      }
    }
    return (titles, timestamps, cids);
  }
}
