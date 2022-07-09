cmake_minimum_required (VERSION 3.8)

# Add source to this project's executable.
add_executable( ${PROJECT_NAME}
	"main.c"
)

add_library(librust_interop STATIC IMPORTED)
set_target_properties(librust_interop PROPERTIES IMPORTED_LOCATION ~/repos/rust_interop/target/debug/librust_interop.a)

target_link_libraries( ${PROJECT_NAME}
    librust_interop
)
