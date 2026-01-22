# Briefcase Installer for Windows
param(
    [string]$InstallPath = "$env:ProgramFiles\Briefcase"
)

Write-Host "Briefcase Backup Tool Installer"
Write-Host "==============================="

# Detect architecture
$arch = $env:PROCESSOR_ARCHITECTURE
if ($arch -eq "AMD64") {
    $binary = "briefcase-windows-x64.zip"
} else {
    Write-Host "Unsupported architecture: $arch"
    exit 1
}

# Download URL (replace with actual GitHub release URL)
$downloadUrl = "https://github.com/br8km/briefcase/releases/latest/download/$binary"

Write-Host "Downloading $binary for Windows $arch..."
Invoke-WebRequest -Uri $downloadUrl -OutFile $binary

Write-Host "Extracting..."
Expand-Archive -Path $binary -DestinationPath $InstallPath -Force

# Add to PATH
$currentPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::Machine)
if ($currentPath -notlike "*$InstallPath*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$InstallPath", [EnvironmentVariableTarget]::Machine)
    Write-Host "Added $InstallPath to system PATH"
}

Write-Host "Cleaning up..."
Remove-Item $binary -Force

Write-Host "Installation complete!"
Write-Host "Run 'briefcase --help' to get started."
Write-Host "You may need to restart your terminal for PATH changes to take effect."