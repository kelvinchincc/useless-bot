# /// script
# requires-python = ">=3.14"
# dependencies = [
#     "typer>=0.27.1",
# ]
# ///
import subprocess
import sys
import time

from typer import Typer

app = Typer()


@app.command()
def build_docker_image() -> None:
    """
    Build a Docker image for the project.
    """
    is_mac = sys.platform == "darwin"
    image = "useless-bot"
    snapshot_version = time.strftime("%Y%m%dT%H%M%S")
    platform = "linux/arm64" if is_mac else "linux/amd64"
    output_dir = "target/docker"

    docker_build_command = [
        "docker",
        "buildx",
        "build",
        "--platform",
        platform,
        "-t",
        f"{image}:{snapshot_version}",
        "--output",
        f"type=docker,dest={output_dir}/{image}-{snapshot_version}.tar",
        ".",
    ]

    if is_mac:
        print(f"Detected macOS. Building for platform {platform}...")
    else:
        print(f"Detected non-macOS. Building for platform {platform}...")

    print(f"Building Docker image with tag {image}:{snapshot_version}...")
    _ = subprocess.run(docker_build_command, check=True)


app()
