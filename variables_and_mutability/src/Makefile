# Files and directories to clean
CLEAN_FILES = *.exe *.pdb
CLEAN_DIRS = target/

# Detect OS and set the correct commands
ifeq ($(OS),Windows_NT)
    RM = cmd /c del /Q
    RMDIR = cmd /c rmdir /S /Q
else
    RM = rm -f
    RMDIR = rm -rf
endif

# Default target
all: clean

# Clean up files and directories
clean:
	-$(RM) $(CLEAN_FILES) 2> NUL
	-$(RMDIR) $(CLEAN_DIRS) 2> NUL
