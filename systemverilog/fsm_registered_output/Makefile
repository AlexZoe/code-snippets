#-------------------------------------------------------------------------------
#
#  @author	Alexander Zoellner
#  @date	2020/02/04
#  @mail	zoellner.contact<at>gmail.com
#  @file	Makefile-systemverilog
#
#  brief	Makefile template for simulating a sytemverilog-based hardware
#			design using icarus verilog and gtkwave.
#
#			Adds verilog and systemverilog files to simulation source files.
#
#-------------------------------------------------------------------------------

# Main verilog file without file ending and testbench topmodule
PROJECT		:= traffic_light

# Testbench writes output to this file instead of stdout
LOGFILE		:= simulation.log

# Directory containing verilog source files
SRC_DIR		:= hdl
# Directory containing testbench
TB_DIR		:= tb
# Directory to generate files in
BUILD_DIR	:= build

# Add all verilog files for compilation
SRCS		:= $(wildcard $(SRC_DIR)/*.sv) $(wildcard $(SRC_DIR)/*.v)
# Add testbench (Only allowed to contain a single testbench file)
TB			:= $(wildcard $(TB_DIR)/*.sv)

# Simulator to use (icarus verilog)
SIM			:= iverilog
# Flags for icarus verilog (uses systemverilog 2012 standard)
SIM_FLAGS	:= -gstrict-ca-eval -gio-range-error -Wall \
		       -Wsensitivity-entire-vector -g2012 -Y .sv -Y .v

# Targets
.PHONY: clean sim build build_dir wave
# Default when calling 'make'
.DEFAULT_GOAL := build

# Build testbench executable
build: build_dir
	@echo '    ' BUILD $(PROJECT)
	@$(SIM) $(SIM_FLAGS) -o $(BUILD_DIR)/$(PROJECT)_tb -s $(PROJECT)_tb $(TB) $(SRCS)

# Execute testbench and generate GtKWave input file
sim: build
	@echo '    ' SIMULATE $(PROJECT)
	@cd $(BUILD_DIR) && vvp $(PROJECT)_tb > $(LOGFILE)

# Start GTKWave with save file (requires manually running 'make sim' first)
wave:
	@gtkwave --save=wave.gtkw $(BUILD_DIR)/*.vcd

# Create build directory if it does not exist
build_dir:
	@mkdir -p $(BUILD_DIR)

# Remove all generated files
clean:
	@echo '    ' CLEAN $(BUILD_DIR)
	@rm -rf $(BUILD_DIR)/$(PROJECT)_tb
	@rm -rf $(BUILD_DIR)/*.vcd
	@rm -rf $(BUILD_DIR)/*.vvp
	@rm -rf $(BUILD_DIR)/*.fst
	@rm -rf $(BUILD_DIR)/*.log
	@rm -rf $(BUILD_DIR)/*.wave
	@rm -rf $(BUILD_DIR)
