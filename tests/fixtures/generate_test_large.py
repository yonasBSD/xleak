#!/usr/bin/env python3
"""
Generate large Excel file for testing xleak's lazy loading performance.
Run: python3 generate_test_large.py [num_rows]
Default: 10,000 rows
"""

try:
    from openpyxl import Workbook
    import random
    import time
except ImportError:
    print("Please install openpyxl: pip install openpyxl")
    exit(1)

def create_large_file(num_rows=10000):
    """Create large test file with specified number of rows."""
    print(f"📊 Generating test_large.xlsx with {num_rows:,} rows...")
    start_time = time.time()

    wb = Workbook()
    ws = wb.active
    ws.title = "LargeData"

    # Headers
    headers = ["ID", "Name", "Email", "Age", "City", "Country", "Salary",
               "Department", "Status", "JoinDate", "Score", "Rating"]
    ws.append(headers)

    # Sample data pools
    first_names = ["John", "Jane", "Michael", "Sarah", "David", "Emma", "Chris", "Lisa"]
    last_names = ["Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis"]
    cities = ["New York", "Los Angeles", "Chicago", "Houston", "Phoenix", "Philadelphia"]
    countries = ["USA", "Canada", "UK", "Germany", "France", "Japan", "Australia"]
    departments = ["Engineering", "Sales", "Marketing", "HR", "Finance", "Operations"]
    statuses = ["Active", "Inactive", "Pending"]

    print("⏳ Generating rows (this may take a minute)...")

    for i in range(1, num_rows + 1):
        if i % 1000 == 0:
            print(f"  Generated {i:,} rows...")

        row_data = [
            i,  # ID
            f"{random.choice(first_names)} {random.choice(last_names)}",  # Name
            f"user{i}@example.com",  # Email
            random.randint(22, 65),  # Age
            random.choice(cities),  # City
            random.choice(countries),  # Country
            random.randint(40000, 150000),  # Salary
            random.choice(departments),  # Department
            random.choice(statuses),  # Status
            f"2020-{random.randint(1,12):02d}-{random.randint(1,28):02d}",  # JoinDate
            round(random.uniform(60, 100), 1),  # Score
            round(random.uniform(1, 5), 1),  # Rating
        ]
        ws.append(row_data)

    # Create a smaller sheet for comparison and multi-sheet testing
    ws2 = wb.create_sheet("SmallData")
    ws2.append(["Product", "Price", "Stock"])
    for i in range(50):
        ws2.append([f"Product {i+1}", round(random.uniform(10, 500), 2), random.randint(0, 1000)])

    filename = "test_large.xlsx"
    print(f"💾 Saving {filename}...")
    wb.save(filename)

    elapsed = time.time() - start_time
    print(f"\n✓ Created {filename}")
    print(f"   - LargeData sheet: {num_rows:,} rows × {len(headers)} columns")
    print(f"   - SmallData sheet: 50 rows × 3 columns")
    print(f"   - Generation time: {elapsed:.1f} seconds")
    print()
    print("Test with:")
    print(f"  ./target/release/xleak {filename} -i")
    print(f"  ./target/release/xleak {filename} --sheet LargeData -n 20")

if __name__ == "__main__":
    import sys

    # Allow custom row count
    num_rows = 10000
    if len(sys.argv) > 1:
        try:
            num_rows = int(sys.argv[1])
        except ValueError:
            print("Usage: python3 generate_test_large.py [num_rows]")
            print("Example: python3 generate_test_large.py 50000")
            sys.exit(1)

    create_large_file(num_rows)
