export class WalletConnector {
  private address: string | null = null;
  private provider: any = null;

  async connect(walletType: 'petra' | 'martian' | 'metamask' = 'petra'): Promise<string> {
    if (typeof window === 'undefined') {
      throw new Error('Wallet connection only available in browser');
    }

    switch (walletType) {
      case 'petra':
        return await this.connectPetra();
      case 'martian':
        return await this.connectMartian();
      case 'metamask':
        return await this.connectMetaMask();
      default:
        throw new Error(`Unsupported wallet type: ${walletType}`);
    }
  }

  private async connectPetra(): Promise<string> {
    const petra = (window as any).aptos;
    if (!petra) {
      throw new Error('Petra wallet not installed');
    }

    const response = await petra.connect();
    this.address = response.address;
    this.provider = petra;
    return this.address;
  }

  private async connectMartian(): Promise<string> {
    const martian = (window as any).martian;
    if (!martian) {
      throw new Error('Martian wallet not installed');
    }

    const response = await martian.connect();
    this.address = response.address;
    this.provider = martian;
    return this.address;
  }

  private async connectMetaMask(): Promise<string> {
    const ethereum = (window as any).ethereum;
    if (!ethereum) {
      throw new Error('MetaMask not installed');
    }

    const accounts = await ethereum.request({ 
      method: 'eth_requestAccounts' 
    });
    
    this.address = accounts[0];
    this.provider = ethereum;
    return this.address;
  }

  async disconnect(): Promise<void> {
    if (this.provider?.disconnect) {
      await this.provider.disconnect();
    }
    this.address = null;
    this.provider = null;
  }

  getAddress(): string | null {
    return this.address;
  }

  isConnected(): boolean {
    return this.address !== null;
  }

  async signMessage(message: string): Promise<string> {
    if (!this.provider) {
      throw new Error('Wallet not connected');
    }

    if (this.provider.signMessage) {
      const signature = await this.provider.signMessage(message);
      return signature;
    }

    throw new Error('Wallet does not support message signing');
  }

  async signTransaction(tx: any): Promise<string> {
    if (!this.provider) {
      throw new Error('Wallet not connected');
    }

    if (this.provider.signTransaction) {
      const signedTx = await this.provider.signTransaction(tx);
      return signedTx;
    }

    throw new Error('Wallet does not support transaction signing');
  }
}
