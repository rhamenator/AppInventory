# AppInventory

A fast, tested successor to the unversioned `program-list2csv` script. It reads
UTF-8, UTF-16LE, or UTF-16BE labeled inventories; tolerates missing optional
fields; sorts consistently; escapes CSV correctly; accepts command-line paths;
and fails when no records are found.

```powershell
cargo test
cargo run -- software_list.txt software_summary.csv
```

Next slices: native Windows/Linux/macOS collectors, JSON/SBOM output, duplicate
normalization, inventory comparison, and provenance timestamps.
