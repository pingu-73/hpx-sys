# hpx-sys

Rust bindings to hpx, a Concurrency and Parallelism library.

# Building on Linux
## Dependencies
- boost
- openmpi (optional)

## Prerequisites
Install [Portable Hardware Locality (hwloc)](https://www.open-mpi.org/software/hwloc/v2.11/)
```
tar -xf hwloc-2.11.0.tar.gz && cd hwloc-2.11.0/
mkdir build && cd build
../configure --prefix=/home/pingu/opt/hwloc
make -j
make install
```

Install [gpreftools](https://github.com/gperftools/gperftools)
```
git clone https://github.com/gperftools/gperftools.git
cd gperftools 
./autogen.sh
mkdir build && cd build
../configure --enable-minimal --prefix=/home/pingu/opt/gperftools
make
make install
```

## Building HPX
Get the [hpx](https://github.com/STEllAR-GROUP/hpx/) development branch.
```
git clone https://github.com/STEllAR-GROUP/hpx.git
```
Configure it with CMake
```
cd hpx
mkdir build
cd build
cmake -DCMAKE_INSTALL_PREFIX=/opt/hpx -DBOOST_ROOT=<path to boost> \
-DTCMALLOC_INCLUDE_DIR=/home/pingu/opt/gperftools/include/ \
-DTCMALLOC_LIBRARY=/home/pingu/opt/gperftools/lib/libtcmalloc_minimal_debug.so \
-DHWLOC_LIBRARY=/home/pingu/opt/hwloc/lib/libhwloc.so \
-DHWLOC_INCLUDE_DIR=/home/pingu/opt/hwloc/include/ ..
make -j8 
make install
```

Add the library path of HPX to ldconfig
```
sudo echo /opt/hpx/lib > /etc/ld.so.conf.d/hpx.conf
sudo ldconfig
```
