#include <iostream>
#include <string>
#include <vector>
#include "rocksdb/db.h"
#include "rocksdb/options.h"
#include <assert.h>

using namespace rocksdb;

class MyComparator : public Comparator {
 public:
  // Compare lhs and rhs, taking timestamp, if exists, into consideration.
  int Compare(const Slice& lhs, const Slice& rhs) const override { }
  // Compare two timestamps ts1 and ts2.
  int CompareTimestamp(const Slice& ts1, const Slice& ts2) const override { }
  // Compare a and b after stripping timestamp from them.
  int CompareWithoutTimestamp(const Slice& a, const Slice& b) const override { }
};

std::string db_path = "infodb";
DB* db;
Options options;
	
void load() {
	options.IncreaseParallelism();
	options.OptimizeLevelStyleCompaction();
	options.create_if_missing = true;
	delete db;
}
	
void run() {
	options.create_if_missing = true;		
	Status s = DB::Open(options, db_path, &db);
	assert(s.ok());
	if (!s.ok()) std::cerr << s.ToString() << std::endl;
	delete db;
}

void print_pairs() {
  Iterator* it = db->NewIterator(ReadOptions());
  for (it->SeekToFirst(); it->Valid(); it->Next()) {
		std::cout << it->key().ToString() << ": " << it->value().ToString() << std::endl;
  }
  assert(it->status().ok()); // Check for any errors found during the scan
  delete it;
}

void write_sst() {
	std::string key = "key1";
	std::string val = "val1";
	SstFileWriter sst_file_writer(EnvOptions(), options);
	std::string file_path = "file1.sst";
	Status s = sst_file_writer.Open(file_path);
	if (!s.ok()) {
    printf("Error while opening file %s, Error: %s\n", file_path.c_str(),
           s.ToString().c_str());
	}
	s = sst_file_writer.Put(key, val);
	if (!s.ok()) {
		printf("Error while adding Key: %s, Error: %s\n", key.c_str(),
					 s.ToString().c_str());
	}
	s = sst_file_writer.Finish();
	if (!s.ok()) {
    printf("Error while finishing file %s, Error: %s\n", file_path.c_str(),
           s.ToString().c_str());
	}
}

void clean() {
	delete db;
	DestroyDB(db_path, options);
}

int main(int argc, char* argv[]) {
	if (argc < 2) {
		std::cerr << argv[0] << " [CMD] where [CMD] =\n\t[ load, run, write_sst, clean ]" << std::endl;
		return 1;
	}
	for (int i = 1; i < argc; ++i) {
		std::string arg = argv[i];
		if (arg == "load") {
			load();
			std::cout << "load successful\n";
			return 0;
		} else if (arg == "run") {
			run();
			std::cout << "run successful\n";
			return 0;
		} else if (arg == "clean") {
			destroy();
			std::cout << "clean successful\n";
			return 0;
		} else if (arg == "write_sst") {
			write_sst();
			std::cout << "write_sst successful\n";
		} else {
			std::cerr << "nasa we have a problem\n";
			return 1;
		}
	}
	return 0;
}
