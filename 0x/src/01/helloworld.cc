#include <iostream>
#include <string>
#include <vector>

#include "rocksdb/db.h"
#include "rocksdb/options.h"

using namespace rocksdb;

std::string db_path = "infodb";
DB* db;
Options options;
	
void run() {
	options.IncreaseParallelism();
	options.OptimizeLevelStyleCompaction();
	options.create_if_missing = true;		
	Status s = DB::Open(options, db_path, &db);
	assert(s.ok());

	std::string value;	
  s = db->Get(ReadOptions(), "some_key", &value);
  assert(s.IsNotFound());
}

int main() {
	run();
	
	delete db;
	return 0;
}
