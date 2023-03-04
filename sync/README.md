## Sync:
This is responsible for orchestrating managing content on an EVM based backend and IPFS
It does so by maintaining a record of all content on the backend and IPFS contained in the
target directory.
Eventually, I want it to watch the target directory for changes and automatically sync it by:
- Adding new content:
  - Any content or file that gets added to the target directory will be added to the backend and
  IPFS.
  - The associated metadata and cid will be stored in the manifest file.
- Deleting content:
  - Any content or file that gets deleted from the target directory will be deleted from the backend
  and IPFS.
  - The associated metadata and cid will be removed from the manifest file.
- Updating content:
  - Any content or file that gets modified in the target directory will be updated in the backend.
  - The new file will be added to IPFS. The old file will be maintained in IPFS.
  - The new metadata and cid will be stored in the manifest file.

For now it will just be a CLI that can be run to sync the contents of the target directory with
the backend and IPFS when needed.

## TODOs
- [ ] Upgrade the manifest to be a database
- [ ] Implement the daemon
- [ ] Implement managing local IPFS node
- [ ] Use signing with a Ledger wallet to authenticate with the backend
- [ ] Or implement a delegated authentication scheme using fission: https://guide.fission.codes/accounts/account-signup