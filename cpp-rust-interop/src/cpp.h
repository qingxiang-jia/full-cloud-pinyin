#pragma once
#include <memory>

class BlobstoreClient {
public:
  BlobstoreClient();
};

std::unique_ptr<BlobstoreClient> new_blobstore_client();