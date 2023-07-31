MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables

name = example_usage

src = src
include_dir = include/$(name)
target = target
debug_directory = $(target)/debug
release_directory = $(target)/release
debug_tmp = $(debug_directory)/tmp
debug = $(debug_directory)/$(name)
release = $(release_directory)/$(name)

sources = $(shell find $(src) -name '*.cpp')
headers = $(shell find $(include_dir) -name '*.hpp')

objs = $(sources:$(src)/%.cpp=$(debug_tmp)/%.o)
deps = $(objs:.o=.d)


pauli_tracker = pauli_tracker
pauli_tracker_directory = ../output

pauli_tracker_static = -Wl,-Bstatic -l$(pauli_tracker) -Wl,-Bdynamic
pauli_tracker_dynamic = -l$(pauli_tracker) -Wl,-rpath=$(pauli_tracker_path)


std = -std=c++20
err = -Wall -Wextra -pedantic
op = -O3

lto = -flto

deps_generation = -MMD -MP


CXX = clang++

include_headers = -I $(include_dir) -I $(pauli_tracker_directory)
LD_LIBRARY_PATH = -L $(pauli_tracker_directory)

CPPFLAGS = $(deps_generation)

CFLAGS = $(err) $(std) $(include_headers)

LDLIBS = $(pauli_tracker_static)
# dynamic is usually a bad idea when it is not installed on the system
# LDLIBS = $(pauli_tracker_dynamic)
LDFLAGS = $(LD_LIBRARY_PATH)


.PHONY: clean update_debug_tmp

main: $(debug)

# debug: $(debug)
# release: $(release)


$(debug): $(objs)
	$(CXX) -o $@ $^ $(LDLIBS) $(LDFLAGS)

-include $(deps)

$(debug_tmp)/%.o:
	$(CXX) -c -o $@ $(@:$(debug_tmp)/%.o=$(src)/%.cpp) $(CPPFLAGS) $(CFLAGS)


$(release): $(sources) $(headers)
	$(CXX) -o $@ $(sources) $(CFLAGS) $(op) $(LDFLAGS) $(lto) $(LDLIBS)



update_debug_tmp:
	@mkdir -p $(target)/release
	@for d in $$(find $(src) -type d); do\
		mkdir -p "$$(echo $$d | sed 's|src|$(debug_tmp)|')";\
	done;\
	for d in $$(find $(debug_tmp) -type d); do\
		echo $$d;\
		if [ -z \
			"$$(find $(src) -type d -wholename $$(echo $$d|sed 's|$(debug_tmp)|src|'))"\
		]; then\
			rm -rf $$d;\
		fi;\
	done


clean:
	rm -f $(debug) $(release) $(objs) $(deps)
