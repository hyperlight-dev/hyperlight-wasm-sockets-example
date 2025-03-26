# Cmdlet to send and receive UDP datagrams via stdin and stdout
 
function Send-UDPMessage {
    [CmdletBinding()]
    param (
        [Parameter(Mandatory = $true)]
        [string]$IPAddress, # Target IP address
 
        [Parameter(Mandatory = $true)]
        [int]$Port # Target port
    )
 
    # Create a UDP client
    $udpClient = New-Object System.Net.Sockets.UdpClient
    $remoteEndPoint = New-Object System.Net.IPEndPoint([System.Net.IPAddress]::Any, 0)
 
    try {
        Write-Host "Reading from stdin. Press Ctrl+C to exit."
 
        while ($true) {
            # Read a line from stdin
            $message = [Console]::In.ReadLine()
 
            # Exit if stdin is closed
            if ($null -eq $message) {
                break
            }
 
            # Convert the message to bytes
            $bytes = [System.Text.Encoding]::UTF8.GetBytes($message)
 
            # Send the message
            $udpClient.Send($bytes, $bytes.Length, $IPAddress, $Port) | Out-Null
 
            # Receive the response
            $responseBytes = $udpClient.Receive([ref]$remoteEndPoint)
            $response = [System.Text.Encoding]::UTF8.GetString($responseBytes)
 
            # Write the response to stdout
            [Console]::Out.WriteLine($response)
        }
    } finally {
        # Close the UDP client
        $udpClient.Close()
        Write-Host "UDP client closed."
    }
}
 
# Example usage:
# Send-UDPMessage -IPAddress "127.0.0.1" -Port 8080