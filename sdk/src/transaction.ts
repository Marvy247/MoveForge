import { Transaction } from './types';

export class TransactionBuilder {
  private tx: Partial<Transaction> = {};

  from(address: string): TransactionBuilder {
    this.tx.from = address;
    return this;
  }

  to(address: string): TransactionBuilder {
    this.tx.to = address;
    return this;
  }

  value(amount: string): TransactionBuilder {
    this.tx.value = amount;
    return this;
  }

  gasLimit(limit: number): TransactionBuilder {
    this.tx.gasLimit = limit;
    return this;
  }

  data(data: string): TransactionBuilder {
    this.tx.data = data;
    return this;
  }

  type(type: 'transfer' | 'swap' | 'mint' | 'burn' | 'deploy' | 'call'): TransactionBuilder {
    this.tx.type = type;
    return this;
  }

  build(): Transaction {
    if (!this.tx.from || !this.tx.to) {
      throw new Error('Transaction must have from and to addresses');
    }
    
    return {
      from: this.tx.from,
      to: this.tx.to,
      value: this.tx.value || '0',
      gasLimit: this.tx.gasLimit || 21000,
      data: this.tx.data || '',
      type: this.tx.type || 'transfer',
    };
  }

  static createTransfer(from: string, to: string, value: string): Transaction {
    return new TransactionBuilder()
      .from(from)
      .to(to)
      .value(value)
      .type('transfer')
      .build();
  }

  static createSwap(
    from: string,
    pool: string,
    amountIn: string,
    data: string
  ): Transaction {
    return new TransactionBuilder()
      .from(from)
      .to(pool)
      .value(amountIn)
      .data(data)
      .type('swap')
      .gasLimit(85000)
      .build();
  }

  static createMint(from: string, nftContract: string, data: string): Transaction {
    return new TransactionBuilder()
      .from(from)
      .to(nftContract)
      .value('0')
      .data(data)
      .type('mint')
      .gasLimit(120000)
      .build();
  }
}
