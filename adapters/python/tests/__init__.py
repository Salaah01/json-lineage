import os
import sys

# Add the parent directory to the path so we can import the module
sys.path.append(
    os.path.realpath(
        os.path.join(os.path.realpath(__file__), "..", "..", "json_lineage")
    )
)
