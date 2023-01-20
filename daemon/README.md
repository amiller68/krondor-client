## Daemon:
This is responsible for orchestrating managing content on an EVM based backend and IPFS
It does so by maintaining a record of all content on the backend and IPFS contained in the
target directory.
Eventually, I want it to watch the target directory for changes and automatically sync them:
- Any content or file that gets added to the target directory will be added to the backend and
IPFS.
- The associated metadata and cid will be stored in the manifest file.
- Any content or file that gets removed from the target directory will be removed from the
backend, but maintained in IPFS.
- The associated metadata and cid will stay in the manifest file, but will be marked as
deleted.
- Any content or file that gets modified in the target directory will be updated in the backend.
- The new file will be added to IPFS. The old file will be maintained in IPFS.
- The new metadata and cid will be stored in the manifest file. And the old metadata and cid
updated to point to the new metadata and cid.

For now it will just be a CLI that can be run to sync the contents of the target directory with
the backend and IPFS when needed.