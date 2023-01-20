## Welcome to Krondor.org

This website is a place for me to:
- curate my thoughts, ideas, and content to share with others.
- experiment with new technologies and building Web3.0 applications.

The scope of this first iteration is very basic -- it just comprises a blog and an about page, most of which is implemented statically. It should be deployed on Vercel or some other traditional server environment.

I think its a good template for building NextJS applications as well, if not needing a little cleaning up. Feel free to fork it and use it for your own projects.

Go check out [my repository](https://github.com/amiller68/krondor-org)

You might notice that my blog posts are indexed by a Content Identifier (CID), which is a hash of the underlying data.
The first cool thing I'm doing with Web3.0 for this prototype is pinning my blog posts on IPFS using a service called Estuary.
This is cool because it means my underlying content is accessible from any peer running IPFS connected to Estuary, and addressable on the growing decentralized web!

I hope to decentralize more of this as I keep on building. Right now this site is hosted on centralized server, posts are indexed statically, and the upload process is pretty cumbersome. I'm working on solutions to these problems by possibly:
- hosting the site on IPFS, using a service like Fleek
- hosting metadata feeds (think lists of blog posts) on a decentralized database like Ceramic
- writing an accompanying local client that:
    - Writes blog post metadata to a Ceramic database
    - Uploads the blog post to IPFS using an rclone integration between dropbox and Estuary

The goal is to have a framework for writing, publishing, and deploying my content on IPFS, all from the comfort of my terminal :)

Thanks for stopping by, I hope to have more to show soon.