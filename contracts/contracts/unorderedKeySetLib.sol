// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

/// @title UnorderedKeySet Library
/// @author Rob Hitchens
/// @notice Library for managing CRUD operations in dynamic key sets.
/// @dev This library is not tested or audited. Do not use for production.
/// @dev Much thanks to Rob Hitchens for this library.

/*
    Hitchens UnorderedKeySet v0.93

    Library for managing CRUD operations in dynamic key sets.
    https://github.com/rob-Hitchens/UnorderedKeySet

    Copyright (c), 2019, Rob Hitchens, the MIT License
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

    THIS SOFTWARE IS NOT TESTED OR AUDITED. DO NOT USE FOR PRODUCTION.
*/

// Sorry Rob, I renamed the library to UnorderedKeySetLib
library UnorderedKeySetLib {
  // Our core Set struct
  struct Set {
    // Maps keys to their index in the keyList
    mapping(bytes32 => uint) keyPointers;
    // List of keys in the set
    bytes32[] keyList;
  }

  // Insert a key into the set
  function insert(Set storage self, bytes32 key) internal {
    require(key != 0x0, 'UnorderedKeySet(100) - Key cannot be 0x0');
    require(
      !exists(self, key),
      'UnorderedKeySet(101) - Key already exists in the set.'
    );
    self.keyList.push(key);
    self.keyPointers[key] = self.keyList.length - 1;
  }

  // Remove a key from the set
  function remove(Set storage self, bytes32 key) internal {
    require(
      exists(self, key),
      'UnorderedKeySet(102) - Key does not exist in the set.'
    );
    bytes32 keyToMove = self.keyList[count(self) - 1];
    uint rowToReplace = self.keyPointers[key];
    self.keyPointers[keyToMove] = rowToReplace;
    self.keyList[rowToReplace] = keyToMove;
    delete self.keyPointers[key];
    self.keyList.pop();
  }

  // Return the number of keys in the set
  function count(Set storage self) internal view returns (uint) {
    return (self.keyList.length);
  }

  // Check if a key exists in the set
  function exists(Set storage self, bytes32 key) internal view returns (bool) {
    if (self.keyList.length == 0) return false;
    return self.keyList[self.keyPointers[key]] == key;
  }

  // Return the key at a given index
  function keyAtIndex(
    Set storage self,
    uint index
  ) internal view returns (bytes32) {
    return self.keyList[index];
  }

  // Delete the entire set
  function nukeSet(Set storage self) public {
    delete self.keyList;
  }
}
