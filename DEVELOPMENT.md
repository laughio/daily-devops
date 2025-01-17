## Pre-requisites

In any case, you will need:

* Linux, Mac OS X, or Windows on an AMD64 platform (aka `x86_64`)
    ** Mac OS X in ARM64 (M1, …) seems to be tricky, but should work.
* Podman or Docker
    ** Windows containers will not work, you need to use Linux based containers, and again `x86_64`.
* Some tools
    ** git
    ** GNU Make
    ** npm
    ** kubectl
    ** HTTPie 2.2+
* A lot of cores, patience, memory, and disk space.

## Optional requirements

* **Rust+** – By default the build will run inside a container image, with Rust included. So you don't necessarily
  need to install Rust on your local machine. However, having Rust installed might come in handy at some point. If you
  want to use an IDE, that might require a Rust installation. Or if you want to quickly run tests, maybe from inside
  your IDE, then this will require Rust as well.
  +
  In any case, you need to be sure that you install at least the version of Rust mentioned above. If you installed
  Rust using `rustup` and default options, then performing an upgrade should be as easy as running `rustup update`.

* **Kubernetes** - Some form of Kubernetes cluster (~1.25)
  ** **Minikube** is what seems to work best for development, and is easy to get started with.
  ** **Kind** also works, uses less resources, but is less tested.
  ** **OpenShift** also works and make several things easier (like proper DNS names and certs), but is
    also more complex to set up.

* **An IDE** – Whatever works best for you. Eclipse, Emacs, IntelliJ, Vim, … footnote:1[This list is sorted in
  alphabetical order, not in the order of any preference.] should all be usable with this project. We do not require
  any specific IDE. We also do not commit any IDE specific files either.

## Operating system

There are different ways to install the required dependencies on the different operating systems. Some operating
systems also might require some additional settings. This section should help to get you started.

### Fedora

Use an "update to date" version of Fedora. Install the following dependencies:

```
sudo dnf install curl openssl-devel npm gcc gcc-c++ make cyrus-sasl-devel cmake libpq-devel postgresql podman
```

### Windows

Assuming you have Windows 10 and admin access.

Install:

* Git for Windows
* GNU Make 4.x
    * install mingw-w64, as described here: https://code.visualstudio.com/docs/cpp/config-mingw
    * or, install "GNU make" using Chocolatey
* Docker for Windows
    * Enable WSL2

NOTE: Needs more testing

### Mac OS

Most of the required tools you can install using [brew](https://brew.sh/) package manager, e.g.

```
brew install git make
```

Using OpenSSL and Cyrus SASL libraries native is still work in progress, so you should use container build for the time being
as described below.

## Building

While the build is based on `cargo`, the build is still driven by the main `Makefile`, located in
the root of the repository. By default, the cargo build running inside a build container. This reduces
the number of pre-requisites you need to install, and makes it easier on platforms like Windows or Mac OS X.

To perform a full build execute:

```
make build
```

This builds the cargo based projects, the frontend, and the container images.

Builds are done using Docker or Podman container runtimes. Podman is preferred if it is present on the system. However, it is
required that Podman supports bind mounts feature properly which is not the case for all platforms today. In that case you can
force Docker runtime with

```
CONTAINER=docker make build
```

## Testing

To run all tests:

```
make test
```

NOTE: When using podman, you currently cannot use `make test`. You need to revert
to `make container-test`, see below.

TIP: This runs only the unit tests and integration tests. There is also a system test-suite at
https://github.com/drogue-iot/drogue-cloud-testing which tests a running deployment.

### Running test on the host

If you have a full build environment on your machine, you can also execute the tests on the host machine,
rather than forking them off in the build container:

```
make container-test
```

### IDE based testing

You can also run cargo tests directly from your IDE. How this works, depends on your IDE.

However, as tests are compiled and executed on the host machine, the same requirements, as when running
tests on the host machine, apply (see above).

## Publishing images

The locally built images can be published with the Makefile as well. For this you need a location to push to.
You can, for example use [quay.io](https://quay.io). Assuming your username on quay.io is "rodney", and
you did log in using `docker login`, then you could do:

```
make push CONTAINER_REGISTRY=quay.io/rodney
```

## Deploying

### Kubernetes instance

Before you can run the deployment, you will need to have access to a Kubernetes cluster. You can run
local cluster using `minikube`. Make sure that your `minikube` cluster is started with `ingress` addon and that you
run `tunnel` in a separate shell

```
minikube start --cpus 4 --memory 16384 --disk-size 20gb --addons ingress --kubernetes-version 1.25.9
# in a separate terminal, as it keeps running
minikube tunnel
```

### Run the deployment

Once the instance is up, and you have ensured that you can access the cluster with `kubectl`, you can run
the following command to run the deployment:

```
make deploy CONTAINER_REGISTRY=quay.io/rodney
```

If you need to pass additional arguments to the deploy script, you can use `DEPLOY_ARGS` environment variable like:

```
env INSTALL_STRIMZI=false DEPLOY_ARGS="-f deploy/examples/managed_kafka.yaml" make deploy
```

### Helm charts

Helm charts are maintained in the separate repository: https://github.com/drogue-iot/drogue-cloud-helm-charts

They are however included as a git submodule at the `deploy/helm` path. A `deploy` target will initialize the submodule.
If you wish to do it manually run:

```
git submodule update --init
```

Also, to pull changes into the existing workspace run:

```
git submodule foreach git pull origin main
```

## How to …

### … work on the frontend

You will need to have `trunk`, `npm` and `sass` installed, as it will drive parts of the build.

`trunk` can be installed using `cargo`:

```
cargo install trunk
```

Installing `sass` can be done using the following command:

```
npm install -g sass@1.52.3
```

### Backend detection

The frontend needs a way to detect which backend to use. This is done by loading an initial `backend.json`
from the location of the frontend.

For a local development, this file can be provided at `console-frontend/dev/endpoints/backend.json` or be overridden
by `console-frontend/dev/endpoints/backend.local.json`. But default files with `.local.` in that directory will not
be committed to git. The default `backend.json` is pre-configured to use the "local server" mode, described in the
next section.

### Running with a local server

The simplest way to run the `console-backend` is to use [`drogue-cloud-server`](https://github.com/drogue-iot/drogue-cloud/tree/main/server).

```
cd server
cargo run -- --enable-all
```

Once you have it running (bound to localhost which is the default), you can start the `console-frontend` in the
development mode:

```
cd console-frontend
trunk serve
```

### Running with a cloud backend

You can also run the frontend with a backend in the cloud (or local cluster, e.g. minikube).
To do so, you can create a `console-frontend/dev/endpoints/backend.local.json` file and populate it with the API and SSO urls of your drogue instance.

For example (devbox):

```
{
  "url": "https://api-drogue-dev.apps.wonderful.iot-playground.org/",
  "openid": {
    "client_id": "drogue",
    "issuer_url": "https://sso-drogue-dev.apps.wonderful.iot-playground.org/realms/drogue"
  }
}
```

NOTE: This model doesn't work if your frontend will use newer backend APIs, which are not yet deployed in the cloud.
