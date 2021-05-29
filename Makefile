test:
	@"build/lucerna.exe" examples/count.sic
	@"build/lucerna.exe" examples/multiply.sic
	@"build/lucerna.exe" examples/smaller.sic
	@"build/lucerna.exe" examples/substract.sic
	@"build/lucerna.exe" examples/error.sic
	@"build/lucerna.exe" examples/web.sic
b:
	@"build.bat"

btest:
	@"build.bat"
	@"build/lucerna.exe" examples/count.sic
	@"build/lucerna.exe" examples/multiply.sic
	@"build/lucerna.exe" examples/smaller.sic
	@"build/lucerna.exe" examples/substract.sic
	@"build/lucerna.exe" examples/error.sic
