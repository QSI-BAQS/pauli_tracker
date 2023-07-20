MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables

.PHONY: clean

all: main

CXX = clang++

STD = -std=c++20
ERR = -Wall -Wextra -pedantic
OP = -O3

# link = -fuse-ld=lld
link =

# lto = -flto=3  # parallize using 3 jobs (default is 1)
lto = -flto
# lto = 

tracker_path = ../tracker/build/output
tracker = -Wl,-Bstatic -ltracker -Wl,-Bdynamic

INCLUDE = -I $(tracker_path)
LD_LIBRARY_PATH = -L $(tracker_path)

CPPFLAGS =

CFLAGS = $(OP) $(ERR) $(STD) $(INCLUDE)

LDLIBS = $(tracker)
LDFLAGS = $(link) $(lto) $(LD_LIBRARY_PATH)

%: %.cpp $(tracker_path)/tracker.h
	$(CXX) -o $@ $< $(CPPFLAGS) $(CFLAGS) $(LDFLAGS) $(LDLIBS) 


clean:
	rm -f main
