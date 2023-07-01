import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
from collections import namedtuple


def prep_datasets(sizes, json_dataset, lineage_dataset):
    regression_json = np.polyfit(sizes, json_dataset, 1)
    regression_lineage = np.polyfit(sizes, lineage_dataset, 1)

    line_of_best_fit_json = np.polyval(regression_json, sizes)
    line_of_best_fit_lineage = np.polyval(regression_lineage, sizes)

    return namedtuple(
        "Datasets",
        [
            "sizes",
            "json_dataset",
            "lineage_dataset",
            "line_of_best_fit_json",
            "line_of_best_fit_lineage",
        ],
    )(
        sizes,
        json_dataset,
        lineage_dataset,
        line_of_best_fit_json,
        line_of_best_fit_lineage,
    )


def create_time_diff_chart(df):
    dataset = prep_datasets(
        df["Size (MB)"],
        df["json time (s)"],
        df["json_lineage time (s)"],
    )
    plt = create_chart(
        dataset, "Time Difference Between JSON and JSON Lineage", "Time (s)"
    )
    plt.savefig("time_diff_chart.png")


def create_mem_diff_chart(df):
    dataset = prep_datasets(
        df["Size (MB)"],
        df["json memory (MB)"],
        df["json_lineage memory (MB)"],
    )
    plt = create_chart(
        dataset,
        "Memory Difference Between JSON and JSON Lineage",
        "Memory (MB)",
    )
    plt.savefig("mem_diff_chart.png")


def create_chart(dataset, title, y_label):
    # Calculate the lines of best fit
    regression_json = np.polyfit(dataset.sizes, dataset.json_dataset, 1)
    regression_lineage = np.polyfit(dataset.sizes, dataset.lineage_dataset, 1)

    line_of_best_fit_json = np.polyval(regression_json, dataset.sizes)
    line_of_best_fit_lineage = np.polyval(regression_lineage, dataset.sizes)

    # Create the scatter plot
    plt.figure(figsize=(8, 6))
    sns.scatterplot(x=dataset.sizes, y=dataset.json_dataset, label="JSON Time")
    plt.plot(
        dataset.sizes,
        line_of_best_fit_json,
        label="JSON (Line of Best Fit)",
        color="#3571A3",
    )
    sns.scatterplot(
        x=dataset.sizes, y=dataset.lineage_dataset, label="JSON Lineage"
    )
    plt.plot(
        dataset.sizes,
        line_of_best_fit_lineage,
        label="JSON Lineage (Line of Best Fit)",
        color="#f57f00",
    )

    # Set labels and title
    plt.xlabel("Size (MB)")
    plt.ylabel(y_label)
    plt.title(title)

    # Add legend
    plt.legend()

    # Save the plot as a PNG file
    return plt


def main():
    df = pd.read_csv("data.csv")
    create_time_diff_chart(df)
    create_mem_diff_chart(df)


if __name__ == "__main__":
    main()
