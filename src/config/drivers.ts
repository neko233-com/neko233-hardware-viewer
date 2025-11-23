export const driverLinks: Record<string, string> = {
  // GPU
  'NVIDIA': 'https://www.nvidia.com/Download/index.aspx',
  'AMD': 'https://www.amd.com/en/support',
  'Intel': 'https://www.intel.com/content/www/us/en/download-center/home.html',
  
  // Motherboard / Chipset
  'ASUS': 'https://www.asus.com/support/Download-Center/',
  'Gigabyte': 'https://www.gigabyte.com/Support',
  'MSI': 'https://www.msi.com/support/download',
  'ASRock': 'https://www.asrock.com/support/index.asp',
  
  // Storage
  'Samsung': 'https://semiconductor.samsung.com/consumer-storage/support/tools/',
  'Western Digital': 'https://support-en.wd.com/app/downloads',
  'Seagate': 'https://www.seagate.com/support/downloads/',
  
  // General fallback
  'Microsoft': 'https://support.microsoft.com/en-us/windows/update-drivers-manually-in-windows-ec62f46c-ff14-c91d-eead-d7126dc1f7b6'
};

export const getDriverLink = (manufacturer: string, productName: string): string | null => {
  if (!manufacturer) return null;
  
  for (const key in driverLinks) {
    if (manufacturer.toLowerCase().includes(key.toLowerCase()) || productName.toLowerCase().includes(key.toLowerCase())) {
      return driverLinks[key];
    }
  }
  
  return null;
};
