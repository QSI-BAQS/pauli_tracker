MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables

name = app

src = src
include_public = include/$(name)
include_private := include/private
target = target
tmp_build = $(target)/build
sources = $(shell find $(src) -name '*.cpp')
headers_public = $(shell find $(include_public) -name '*.hpp')
# headers_private = $(shell find $(include_private) -name '*.hpp')
headers_private =
objs = $(sources:$(src)/%.cpp=$(tmp_build)/%.o)
deps = $(objs:.o=.d)

tracker_path = ../tracker/build/output
tracker = -Wl,-Bstatic -ltracker -Wl,-Bdynamic

headers = $(headers_public) $(headers_private) $(tracker_path)/tracker.h

std = -std=c++20
err = -Wall -Wextra -pedantic
op = -O3

deps_generation = -MMD -MP

# link = -fuse-ld=lld
link =

# lto = -flto=3  # parallize using 3 jobs (default is 1)
lto = -flto
# lto = 


CXX = clang++

INCLUDE = -I $(include_public) -I $(tracker_path)
# INCLUDE = -I $(include_public) -I $(include_private) -I $(tracker_path)
LD_LIBRARY_PATH = -L $(tracker_path)

CPPFLAGS = $(deps_generation)

CFLAGS = $(err) $(std) $(INCLUDE)

LDLIBS = $(tracker)
LDFLAGS = $(link) $(LD_LIBRARY_PATH)


.PHONY: clean update_build_directory

debug = $(target)/$(name)
release = $(target)/release/$(name)

main: $(debug)


$(debug): $(objs)
	@$(CXX) -o $@ $^ $(LDFLAGS) $(LDLIBS)

-include $(deps)

$(tmp_build)/%.o:
	$(CXX) -c -o $@ $(@:$(tmp_build)/%.o=$(src)/%.cpp) $(CPPFLAGS) $(CFLAGS)


$(release): $(sources) $(headers)
	$(CXX) -o $@ $(sources) $(CFLAGS) $(op) $(LDFLAGS) $(lto) $(LDLIBS)


update_build_directory:
	@mkdir -p $(target)/release
	@for d in $$(find $(src) -type d); do\
		mkdir -p "$$(echo $$d | sed 's|src|$(tmp_build)|')";\
	done;\
	for d in $$(find $(tmp_build) -type d); do\
		echo $$d;\
		if [ -z \
			"$$(find $(src) -type d -wholename $$(echo $$d|sed 's|$(tmp_build)|src|'))"\
		]; then\
			rm -rf $$d;\
		# elif ! [ -f $$d/.gitkeep ]; then\
		# 	touch $$d/.gitkeep;\
		fi;\
	done


clean:
	@rm -f $(debug) $(release) $(objs) $(deps)
