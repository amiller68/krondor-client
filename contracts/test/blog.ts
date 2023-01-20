import { time, loadFixture } from '@nomicfoundation/hardhat-network-helpers';
import { expect } from 'chai';
// @ts-ignore
import { ethers } from 'hardhat';

describe('Blog', function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deployBlogFixture() {
    // Contracts are deployed using the first signer/account by default
    const [owner, otherAccount] = await ethers.getSigners();

    const Blog = await ethers.getContractFactory('Blog');
    const blog = await Blog.deploy();

    return { owner, otherAccount, blog };
  }

  // Test whether the blog is deployed correctly
  describe('Deployment', function () {
    it('Should set the right owner', async function () {
      const { owner, blog } = await loadFixture(deployBlogFixture);

      expect(await blog.owner()).to.equal(owner.address);
    });

    it('Should not have any posts', async function () {
      const { blog } = await loadFixture(deployBlogFixture);
      // Check the post count
      expect(Number(await blog.getPostCount())).to.equal(0);
    });

    it('Should not be writable by other accounts', async function () {
      const { otherAccount, blog } = await loadFixture(deployBlogFixture);

      // Try to write a post
      await expect(
        blog.connect(otherAccount).createPost('Hello World', 'CID123')
        // @ts-ignore - chai-matchers doesn't get picked up, but this should work
      ).to.be.revertedWith('Only the owner can call this function.');
    });
  });

  // Test whether the blog is writable correctly
  describe('Managing content', async function () {
    let defaultTitle: string = 'Hello World';
    let defaultCid: string = 'QmV4Y2hhbmdl';
    let updatedTitle: string = 'Hello World 2';
    let updatedCid: string = 'QmV4Y2hhbmdlMg';

    // It should create, update and delete a post
    it('Should create a post', async function () {
      const { blog } = await loadFixture(deployBlogFixture);
      // Get the post count before
      const postCountBefore = Number(await blog.getPostCount());
      // Create a new post and get the postId from the emitted event
      const tx = await blog.createPost(defaultTitle, defaultCid);
      const receipt = await tx.wait();
      const postId = Number(receipt.events?.[0].args?.postId);
      const timestamp = Number(receipt.events?.[0].args?.timestamp);
      // Check the post count
      expect(Number(await blog.getPostCount())).to.equal(postCountBefore + 1);
      // Check the post data. Returns a tuple of (title, timestamp, cid)
      let post = await blog.getPost(postId);
      expect(post[0]).to.equal(defaultTitle);
      expect(Number(post[1])).to.be.closeTo(timestamp, 1);
      expect(post[2]).to.equal(defaultCid);
    });

    it('Should update a post', async function () {
      const { blog } = await loadFixture(deployBlogFixture);
      // Create a new post and get the postId from the emitted event
      const tx = await blog.createPost(defaultTitle, defaultCid);
      const receipt = await tx.wait();
      const postId = Number(receipt.events?.[0].args?.postId);
      // Update the post
      await blog.updatePost(postId, updatedTitle, updatedCid);
      // Get the post, parsing the returned string as a tuple
      let post = await blog.getPost(postId);
      // Check the post. Results returned as a tuple of (title, timestamp, cid)
      expect(post[0]).to.equal(updatedTitle);
      expect(post[2]).to.equal(updatedCid);
    });

    it('Should delete a post', async function () {
      const { blog } = await loadFixture(deployBlogFixture);
      // Get the post count
      let postCount = Number(await blog.getPostCount());
      // Create a new post and get the postId from the emitted event
      const tx = await blog.createPost(defaultTitle, defaultCid);
      const receipt = await tx.wait();
      const postId = Number(receipt.events?.[0].args?.postId);
      // Delete the post
      await blog.deletePost(postId);
      // Check the post count
      expect(Number(await blog.getPostCount())).to.equal(postCount);
      // Check the post is deleted
      // @ts-ignore - chai-matchers doesn't get picked up, but this should work
      await expect(blog.getPost(postId)).to.be.revertedWith(
        'Post does not exist.'
      );
    });
  });

  // Test whether the blog is readable correctly
  describe('Reading content', function () {
    it('Should read all posts', async function () {
      const { blog } = await loadFixture(deployBlogFixture);
      // Get the post count before
      const postCountBefore = Number(await blog.getPostCount());
      // Create three new posts. Don't need to get the postId from the emitted event
      await blog.createPost('Hello World', 'CID123');
      await blog.createPost('Hello World 2', 'CID123');
      await blog.createPost('Hello World 3', 'CID123');
      // See if we have the right post count
      expect(await blog.getPostCount()).to.equal(postCountBefore + 3);
      // Get the posts
      let posts = await blog.getPostsByPage(0, 3);
      // Check the post content. Ignore timestamp. Results returned a tuple of lists of type ([]string, []uint, []string)
      expect(posts[0][0]).to.equal('Hello World');
      expect(posts[0][1]).to.equal('Hello World 2');
      expect(posts[0][2]).to.equal('Hello World 3');
      expect(posts[2][0]).to.equal('CID123');
      expect(posts[2][1]).to.equal('CID123');
      expect(posts[2][2]).to.equal('CID123');
    });
  });
});
