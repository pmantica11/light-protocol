export type LightCompressedToken = {
    version: '0.3.4';
    name: 'light_compressed_token';
    constants: [
        {
            name: 'PROGRAM_ID';
            type: 'string';
            value: '"HXVfQ44ATEi9WBKLSCCwM54KokdkzqXci9xCQ7ST9SYN"';
        },
    ];
    instructions: [
        {
            name: 'createMint';
            docs: [
                'This instruction expects a mint account to be created in a separate',
                'token program instruction with token authority as mint authority. This',
                'instruction creates a token pool account for that mint owned by token',
                'authority.',
            ];
            accounts: [
                {
                    name: 'feePayer';
                    isMut: true;
                    isSigner: true;
                },
                {
                    name: 'tokenPoolPda';
                    isMut: true;
                    isSigner: false;
                },
                {
                    name: 'systemProgram';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'mint';
                    isMut: true;
                    isSigner: false;
                },
                {
                    name: 'tokenProgram';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'cpiAuthorityPda';
                    isMut: false;
                    isSigner: false;
                },
            ];
            args: [];
        },
        {
            name: 'mintTo';
            docs: [
                'Mints tokens from an spl token mint to a list of compressed accounts.',
                'Minted tokens are transferred to a pool account owned by the compressed',
                'token program. The instruction creates one compressed output account for',
                'every amount and pubkey input pair one output compressed account.',
            ];
            accounts: [
                {
                    name: 'feePayer';
                    isMut: true;
                    isSigner: true;
                },
                {
                    name: 'authority';
                    isMut: false;
                    isSigner: true;
                },
                {
                    name: 'cpiAuthorityPda';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'mint';
                    isMut: true;
                    isSigner: false;
                },
                {
                    name: 'tokenPoolPda';
                    isMut: true;
                    isSigner: false;
                },
                {
                    name: 'tokenProgram';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'lightSystemProgram';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'registeredProgramPda';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'noopProgram';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'accountCompressionAuthority';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'accountCompressionProgram';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'merkleTree';
                    isMut: true;
                    isSigner: false;
                },
                {
                    name: 'selfProgram';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'systemProgram';
                    isMut: false;
                    isSigner: false;
                },
            ];
            args: [
                {
                    name: 'publicKeys';
                    type: {
                        vec: 'publicKey';
                    };
                },
                {
                    name: 'amounts';
                    type: {
                        vec: 'u64';
                    };
                },
            ];
        },
        {
            name: 'transfer';
            accounts: [
                {
                    name: 'feePayer';
                    isMut: true;
                    isSigner: true;
                },
                {
                    name: 'authority';
                    isMut: false;
                    isSigner: true;
                },
                {
                    name: 'cpiAuthorityPda';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'lightSystemProgram';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'registeredProgramPda';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'noopProgram';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'accountCompressionAuthority';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'accountCompressionProgram';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'selfProgram';
                    isMut: false;
                    isSigner: false;
                },
                {
                    name: 'tokenPoolPda';
                    isMut: true;
                    isSigner: false;
                    isOptional: true;
                },
                {
                    name: 'compressOrDecompressTokenAccount';
                    isMut: true;
                    isSigner: false;
                    isOptional: true;
                },
                {
                    name: 'tokenProgram';
                    isMut: false;
                    isSigner: false;
                    isOptional: true;
                },
                {
                    name: 'systemProgram';
                    isMut: false;
                    isSigner: false;
                },
            ];
            args: [
                {
                    name: 'inputs';
                    type: 'bytes';
                },
            ];
        },
    ];
    types: [
        {
            name: 'InstructionDataInvoke';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'proof';
                        type: {
                            option: {
                                defined: 'CompressedProof';
                            };
                        };
                    },
                    {
                        name: 'inputCompressedAccountsWithMerkleContext';
                        type: {
                            vec: {
                                defined: 'PackedCompressedAccountWithMerkleContext';
                            };
                        };
                    },
                    {
                        name: 'outputCompressedAccounts';
                        type: {
                            vec: {
                                defined: 'OutputCompressedAccountWithPackedContext';
                            };
                        };
                    },
                    {
                        name: 'relayFee';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'newAddressParams';
                        type: {
                            vec: {
                                defined: 'NewAddressParamsPacked';
                            };
                        };
                    },
                    {
                        name: 'compressOrDecompressLamports';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'isCompress';
                        type: 'bool';
                    },
                ];
            };
        },
        {
            name: 'OutputCompressedAccountWithContext';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'compressedAccount';
                        type: {
                            defined: 'CompressedAccount';
                        };
                    },
                    {
                        name: 'merkleTree';
                        type: 'publicKey';
                    },
                ];
            };
        },
        {
            name: 'OutputCompressedAccountWithPackedContext';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'compressedAccount';
                        type: {
                            defined: 'CompressedAccount';
                        };
                    },
                    {
                        name: 'merkleTreeIndex';
                        type: 'u8';
                    },
                ];
            };
        },
        {
            name: 'NewAddressParamsPacked';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'seed';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'addressQueueAccountIndex';
                        type: 'u8';
                    },
                    {
                        name: 'addressMerkleTreeAccountIndex';
                        type: 'u8';
                    },
                    {
                        name: 'addressMerkleTreeRootIndex';
                        type: 'u16';
                    },
                ];
            };
        },
        {
            name: 'NewAddressParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'seed';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'addressQueuePubkey';
                        type: 'publicKey';
                    },
                    {
                        name: 'addressMerkleTreePubkey';
                        type: 'publicKey';
                    },
                    {
                        name: 'addressMerkleTreeRootIndex';
                        type: 'u16';
                    },
                ];
            };
        },
        {
            name: 'CompressedProof';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'a';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'b';
                        type: {
                            array: ['u8', 64];
                        };
                    },
                    {
                        name: 'c';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                ];
            };
        },
        {
            name: 'InstructionDataInvokeCpi';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'proof';
                        type: {
                            option: {
                                defined: 'CompressedProof';
                            };
                        };
                    },
                    {
                        name: 'newAddressParams';
                        type: {
                            vec: {
                                defined: 'NewAddressParamsPacked';
                            };
                        };
                    },
                    {
                        name: 'inputCompressedAccountsWithMerkleContext';
                        type: {
                            vec: {
                                defined: 'PackedCompressedAccountWithMerkleContext';
                            };
                        };
                    },
                    {
                        name: 'outputCompressedAccounts';
                        type: {
                            vec: {
                                defined: 'OutputCompressedAccountWithPackedContext';
                            };
                        };
                    },
                    {
                        name: 'relayFee';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'compressOrDecompressLamports';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'isCompress';
                        type: 'bool';
                    },
                    {
                        name: 'signerSeeds';
                        type: {
                            vec: 'bytes';
                        };
                    },
                    {
                        name: 'cpiContext';
                        type: {
                            option: {
                                defined: 'CompressedCpiContext';
                            };
                        };
                    },
                ];
            };
        },
        {
            name: 'PackedCompressedAccountWithMerkleContext';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'compressedAccount';
                        type: {
                            defined: 'CompressedAccount';
                        };
                    },
                    {
                        name: 'merkleContext';
                        type: {
                            defined: 'PackedMerkleContext';
                        };
                    },
                    {
                        name: 'rootIndex';
                        docs: [
                            'Index of root used in inclusion validity proof.',
                        ];
                        type: 'u16';
                    },
                ];
            };
        },
        {
            name: 'CompressedAccountWithMerkleContext';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'compressedAccount';
                        type: {
                            defined: 'CompressedAccount';
                        };
                    },
                    {
                        name: 'merkleContext';
                        type: {
                            defined: 'MerkleContext';
                        };
                    },
                ];
            };
        },
        {
            name: 'MerkleContext';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'merkleTreePubkey';
                        type: 'publicKey';
                    },
                    {
                        name: 'nullifierQueuePubkey';
                        type: 'publicKey';
                    },
                    {
                        name: 'leafIndex';
                        type: 'u32';
                    },
                ];
            };
        },
        {
            name: 'PackedMerkleContext';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'merkleTreePubkeyIndex';
                        type: 'u8';
                    },
                    {
                        name: 'nullifierQueuePubkeyIndex';
                        type: 'u8';
                    },
                    {
                        name: 'leafIndex';
                        type: 'u32';
                    },
                ];
            };
        },
        {
            name: 'CompressedAccount';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'owner';
                        type: 'publicKey';
                    },
                    {
                        name: 'lamports';
                        type: 'u64';
                    },
                    {
                        name: 'address';
                        type: {
                            option: {
                                array: ['u8', 32];
                            };
                        };
                    },
                    {
                        name: 'data';
                        type: {
                            option: {
                                defined: 'CompressedAccountData';
                            };
                        };
                    },
                ];
            };
        },
        {
            name: 'CompressedAccountData';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'discriminator';
                        type: {
                            array: ['u8', 8];
                        };
                    },
                    {
                        name: 'data';
                        type: 'bytes';
                    },
                    {
                        name: 'dataHash';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                ];
            };
        },
        {
            name: 'MerkleTreeSequenceNumber';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'pubkey';
                        type: 'publicKey';
                    },
                    {
                        name: 'seq';
                        type: 'u64';
                    },
                ];
            };
        },
        {
            name: 'PublicTransactionEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'inputCompressedAccountHashes';
                        type: {
                            vec: {
                                array: ['u8', 32];
                            };
                        };
                    },
                    {
                        name: 'outputCompressedAccountHashes';
                        type: {
                            vec: {
                                array: ['u8', 32];
                            };
                        };
                    },
                    {
                        name: 'outputCompressedAccounts';
                        type: {
                            vec: {
                                defined: 'OutputCompressedAccountWithPackedContext';
                            };
                        };
                    },
                    {
                        name: 'outputLeafIndices';
                        type: {
                            vec: 'u32';
                        };
                    },
                    {
                        name: 'sequenceNumbers';
                        type: {
                            vec: {
                                defined: 'MerkleTreeSequenceNumber';
                            };
                        };
                    },
                    {
                        name: 'relayFee';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'isCompress';
                        type: 'bool';
                    },
                    {
                        name: 'compressOrDecompressLamports';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'pubkeyArray';
                        type: {
                            vec: 'publicKey';
                        };
                    },
                    {
                        name: 'message';
                        type: {
                            option: 'bytes';
                        };
                    },
                ];
            };
        },
        {
            name: 'CompressedCpiContext';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'setContext';
                        type: 'bool';
                    },
                    {
                        name: 'cpiContextAccountIndex';
                        type: 'u8';
                    },
                ];
            };
        },
        {
            name: 'InputTokenDataWithContext';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'amount';
                        type: 'u64';
                    },
                    {
                        name: 'delegateIndex';
                        type: {
                            option: 'u8';
                        };
                    },
                    {
                        name: 'delegatedAmount';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'isNative';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'merkleContext';
                        type: {
                            defined: 'PackedMerkleContext';
                        };
                    },
                    {
                        name: 'rootIndex';
                        type: 'u16';
                    },
                ];
            };
        },
        {
            name: 'CompressedTokenInstructionDataTransfer';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'proof';
                        type: {
                            option: {
                                defined: 'CompressedProof';
                            };
                        };
                    },
                    {
                        name: 'mint';
                        type: 'publicKey';
                    },
                    {
                        name: 'signerIsDelegate';
                        type: 'bool';
                    },
                    {
                        name: 'inputTokenDataWithContext';
                        type: {
                            vec: {
                                defined: 'InputTokenDataWithContext';
                            };
                        };
                    },
                    {
                        name: 'outputCompressedAccounts';
                        type: {
                            vec: {
                                defined: 'PackedTokenTransferOutputData';
                            };
                        };
                    },
                    {
                        name: 'isCompress';
                        type: 'bool';
                    },
                    {
                        name: 'compressOrDecompressAmount';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'cpiContext';
                        type: {
                            option: {
                                defined: 'CompressedCpiContext';
                            };
                        };
                    },
                ];
            };
        },
        {
            name: 'PackedTokenTransferOutputData';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'owner';
                        type: 'publicKey';
                    },
                    {
                        name: 'amount';
                        type: 'u64';
                    },
                    {
                        name: 'lamports';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'merkleTreeIndex';
                        type: 'u8';
                    },
                ];
            };
        },
        {
            name: 'TokenTransferOutputData';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'owner';
                        type: 'publicKey';
                    },
                    {
                        name: 'amount';
                        type: 'u64';
                    },
                    {
                        name: 'lamports';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'merkleTree';
                        type: 'publicKey';
                    },
                ];
            };
        },
        {
            name: 'TokenData';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'mint';
                        docs: ['The mint associated with this account'];
                        type: 'publicKey';
                    },
                    {
                        name: 'owner';
                        docs: ['The owner of this account.'];
                        type: 'publicKey';
                    },
                    {
                        name: 'amount';
                        docs: ['The amount of tokens this account holds.'];
                        type: 'u64';
                    },
                    {
                        name: 'delegate';
                        docs: [
                            'If `delegate` is `Some` then `delegated_amount` represents',
                            'the amount authorized by the delegate',
                        ];
                        type: {
                            option: 'publicKey';
                        };
                    },
                    {
                        name: 'state';
                        docs: ["The account's state"];
                        type: {
                            defined: 'AccountState';
                        };
                    },
                    {
                        name: 'isNative';
                        docs: [
                            'If is_some, this is a native token, and the value logs the rent-exempt',
                            'reserve. An Account is required to be rent-exempt, so the value is',
                            'used by the Processor to ensure that wrapped SOL accounts do not',
                            'drop below this threshold.',
                        ];
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'delegatedAmount';
                        docs: ['The amount delegated'];
                        type: 'u64';
                    },
                ];
            };
        },
        {
            name: 'AccountState';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'Initialized';
                    },
                    {
                        name: 'Frozen';
                    },
                ];
            };
        },
        {
            name: 'ErrorCode';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'PublicKeyAmountMissmatch';
                    },
                    {
                        name: 'SignerCheckFailed';
                    },
                    {
                        name: 'ComputeInputSumFailed';
                    },
                    {
                        name: 'ComputeOutputSumFailed';
                    },
                    {
                        name: 'ComputeCompressSumFailed';
                    },
                    {
                        name: 'ComputeDecompressSumFailed';
                    },
                    {
                        name: 'SumCheckFailed';
                    },
                    {
                        name: 'DecompressRecipientUndefinedForDecompress';
                    },
                    {
                        name: 'CompressedPdaUndefinedForDecompress';
                    },
                    {
                        name: 'DeCompressAmountUndefinedForDecompress';
                    },
                    {
                        name: 'CompressedPdaUndefinedForCompress';
                    },
                    {
                        name: 'DeCompressAmountUndefinedForCompress';
                    },
                    {
                        name: 'DelegateUndefined';
                    },
                    {
                        name: 'DelegateSignerCheckFailed';
                    },
                    {
                        name: 'SplTokenSupplyMismatch';
                    },
                    {
                        name: 'HeapMemoryCheckFailed';
                    },
                ];
            };
        },
    ];
    errors: [
        {
            code: 6000;
            name: 'SignerCheckFailed';
            msg: 'Signer check failed';
        },
        {
            code: 6001;
            name: 'CreateTransferInstructionFailed';
            msg: 'Create transfer instruction failed';
        },
    ];
};
export const IDL: LightCompressedToken = {
    version: '0.3.4',
    name: 'light_compressed_token',
    constants: [
        {
            name: 'PROGRAM_ID',
            type: 'string',
            value: '"HXVfQ44ATEi9WBKLSCCwM54KokdkzqXci9xCQ7ST9SYN"',
        },
    ],
    instructions: [
        {
            name: 'createMint',
            docs: [
                'This instruction expects a mint account to be created in a separate',
                'token program instruction with token authority as mint authority. This',
                'instruction creates a token pool account for that mint owned by token',
                'authority.',
            ],
            accounts: [
                {
                    name: 'feePayer',
                    isMut: true,
                    isSigner: true,
                },
                {
                    name: 'tokenPoolPda',
                    isMut: true,
                    isSigner: false,
                },
                {
                    name: 'systemProgram',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'mint',
                    isMut: true,
                    isSigner: false,
                },
                {
                    name: 'tokenProgram',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'cpiAuthorityPda',
                    isMut: false,
                    isSigner: false,
                },
            ],
            args: [],
        },
        {
            name: 'mintTo',
            docs: [
                'Mints tokens from an spl token mint to a list of compressed accounts.',
                'Minted tokens are transferred to a pool account owned by the compressed',
                'token program. The instruction creates one compressed output account for',
                'every amount and pubkey input pair one output compressed account.',
            ],
            accounts: [
                {
                    name: 'feePayer',
                    isMut: true,
                    isSigner: true,
                },
                {
                    name: 'authority',
                    isMut: false,
                    isSigner: true,
                },
                {
                    name: 'cpiAuthorityPda',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'mint',
                    isMut: true,
                    isSigner: false,
                },
                {
                    name: 'tokenPoolPda',
                    isMut: true,
                    isSigner: false,
                },
                {
                    name: 'tokenProgram',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'lightSystemProgram',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'registeredProgramPda',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'noopProgram',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'accountCompressionAuthority',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'accountCompressionProgram',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'merkleTree',
                    isMut: true,
                    isSigner: false,
                },
                {
                    name: 'selfProgram',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'systemProgram',
                    isMut: false,
                    isSigner: false,
                },
            ],
            args: [
                {
                    name: 'publicKeys',
                    type: {
                        vec: 'publicKey',
                    },
                },
                {
                    name: 'amounts',
                    type: {
                        vec: 'u64',
                    },
                },
            ],
        },
        {
            name: 'transfer',
            accounts: [
                {
                    name: 'feePayer',
                    isMut: true,
                    isSigner: true,
                },
                {
                    name: 'authority',
                    isMut: false,
                    isSigner: true,
                },
                {
                    name: 'cpiAuthorityPda',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'lightSystemProgram',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'registeredProgramPda',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'noopProgram',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'accountCompressionAuthority',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'accountCompressionProgram',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'selfProgram',
                    isMut: false,
                    isSigner: false,
                },
                {
                    name: 'tokenPoolPda',
                    isMut: true,
                    isSigner: false,
                    isOptional: true,
                },
                {
                    name: 'compressOrDecompressTokenAccount',
                    isMut: true,
                    isSigner: false,
                    isOptional: true,
                },
                {
                    name: 'tokenProgram',
                    isMut: false,
                    isSigner: false,
                    isOptional: true,
                },
                {
                    name: 'systemProgram',
                    isMut: false,
                    isSigner: false,
                },
            ],
            args: [
                {
                    name: 'inputs',
                    type: 'bytes',
                },
            ],
        },
    ],
    types: [
        {
            name: 'InstructionDataInvoke',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'proof',
                        type: {
                            option: {
                                defined: 'CompressedProof',
                            },
                        },
                    },
                    {
                        name: 'inputCompressedAccountsWithMerkleContext',
                        type: {
                            vec: {
                                defined:
                                    'PackedCompressedAccountWithMerkleContext',
                            },
                        },
                    },
                    {
                        name: 'outputCompressedAccounts',
                        type: {
                            vec: {
                                defined:
                                    'OutputCompressedAccountWithPackedContext',
                            },
                        },
                    },
                    {
                        name: 'relayFee',
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'newAddressParams',
                        type: {
                            vec: {
                                defined: 'NewAddressParamsPacked',
                            },
                        },
                    },
                    {
                        name: 'compressOrDecompressLamports',
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'isCompress',
                        type: 'bool',
                    },
                ],
            },
        },
        {
            name: 'OutputCompressedAccountWithContext',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'compressedAccount',
                        type: {
                            defined: 'CompressedAccount',
                        },
                    },
                    {
                        name: 'merkleTree',
                        type: 'publicKey',
                    },
                ],
            },
        },
        {
            name: 'OutputCompressedAccountWithPackedContext',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'compressedAccount',
                        type: {
                            defined: 'CompressedAccount',
                        },
                    },
                    {
                        name: 'merkleTreeIndex',
                        type: 'u8',
                    },
                ],
            },
        },
        {
            name: 'NewAddressParamsPacked',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'seed',
                        type: {
                            array: ['u8', 32],
                        },
                    },
                    {
                        name: 'addressQueueAccountIndex',
                        type: 'u8',
                    },
                    {
                        name: 'addressMerkleTreeAccountIndex',
                        type: 'u8',
                    },
                    {
                        name: 'addressMerkleTreeRootIndex',
                        type: 'u16',
                    },
                ],
            },
        },
        {
            name: 'NewAddressParams',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'seed',
                        type: {
                            array: ['u8', 32],
                        },
                    },
                    {
                        name: 'addressQueuePubkey',
                        type: 'publicKey',
                    },
                    {
                        name: 'addressMerkleTreePubkey',
                        type: 'publicKey',
                    },
                    {
                        name: 'addressMerkleTreeRootIndex',
                        type: 'u16',
                    },
                ],
            },
        },
        {
            name: 'CompressedProof',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'a',
                        type: {
                            array: ['u8', 32],
                        },
                    },
                    {
                        name: 'b',
                        type: {
                            array: ['u8', 64],
                        },
                    },
                    {
                        name: 'c',
                        type: {
                            array: ['u8', 32],
                        },
                    },
                ],
            },
        },
        {
            name: 'InstructionDataInvokeCpi',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'proof',
                        type: {
                            option: {
                                defined: 'CompressedProof',
                            },
                        },
                    },
                    {
                        name: 'newAddressParams',
                        type: {
                            vec: {
                                defined: 'NewAddressParamsPacked',
                            },
                        },
                    },
                    {
                        name: 'inputCompressedAccountsWithMerkleContext',
                        type: {
                            vec: {
                                defined:
                                    'PackedCompressedAccountWithMerkleContext',
                            },
                        },
                    },
                    {
                        name: 'outputCompressedAccounts',
                        type: {
                            vec: {
                                defined:
                                    'OutputCompressedAccountWithPackedContext',
                            },
                        },
                    },
                    {
                        name: 'relayFee',
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'compressOrDecompressLamports',
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'isCompress',
                        type: 'bool',
                    },
                    {
                        name: 'signerSeeds',
                        type: {
                            vec: 'bytes',
                        },
                    },
                    {
                        name: 'cpiContext',
                        type: {
                            option: {
                                defined: 'CompressedCpiContext',
                            },
                        },
                    },
                ],
            },
        },
        {
            name: 'PackedCompressedAccountWithMerkleContext',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'compressedAccount',
                        type: {
                            defined: 'CompressedAccount',
                        },
                    },
                    {
                        name: 'merkleContext',
                        type: {
                            defined: 'PackedMerkleContext',
                        },
                    },
                    {
                        name: 'rootIndex',
                        docs: [
                            'Index of root used in inclusion validity proof.',
                        ],
                        type: 'u16',
                    },
                ],
            },
        },
        {
            name: 'CompressedAccountWithMerkleContext',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'compressedAccount',
                        type: {
                            defined: 'CompressedAccount',
                        },
                    },
                    {
                        name: 'merkleContext',
                        type: {
                            defined: 'MerkleContext',
                        },
                    },
                ],
            },
        },
        {
            name: 'MerkleContext',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'merkleTreePubkey',
                        type: 'publicKey',
                    },
                    {
                        name: 'nullifierQueuePubkey',
                        type: 'publicKey',
                    },
                    {
                        name: 'leafIndex',
                        type: 'u32',
                    },
                ],
            },
        },
        {
            name: 'PackedMerkleContext',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'merkleTreePubkeyIndex',
                        type: 'u8',
                    },
                    {
                        name: 'nullifierQueuePubkeyIndex',
                        type: 'u8',
                    },
                    {
                        name: 'leafIndex',
                        type: 'u32',
                    },
                ],
            },
        },
        {
            name: 'CompressedAccount',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'owner',
                        type: 'publicKey',
                    },
                    {
                        name: 'lamports',
                        type: 'u64',
                    },
                    {
                        name: 'address',
                        type: {
                            option: {
                                array: ['u8', 32],
                            },
                        },
                    },
                    {
                        name: 'data',
                        type: {
                            option: {
                                defined: 'CompressedAccountData',
                            },
                        },
                    },
                ],
            },
        },
        {
            name: 'CompressedAccountData',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'discriminator',
                        type: {
                            array: ['u8', 8],
                        },
                    },
                    {
                        name: 'data',
                        type: 'bytes',
                    },
                    {
                        name: 'dataHash',
                        type: {
                            array: ['u8', 32],
                        },
                    },
                ],
            },
        },
        {
            name: 'MerkleTreeSequenceNumber',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'pubkey',
                        type: 'publicKey',
                    },
                    {
                        name: 'seq',
                        type: 'u64',
                    },
                ],
            },
        },
        {
            name: 'PublicTransactionEvent',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'inputCompressedAccountHashes',
                        type: {
                            vec: {
                                array: ['u8', 32],
                            },
                        },
                    },
                    {
                        name: 'outputCompressedAccountHashes',
                        type: {
                            vec: {
                                array: ['u8', 32],
                            },
                        },
                    },
                    {
                        name: 'outputCompressedAccounts',
                        type: {
                            vec: {
                                defined:
                                    'OutputCompressedAccountWithPackedContext',
                            },
                        },
                    },
                    {
                        name: 'outputLeafIndices',
                        type: {
                            vec: 'u32',
                        },
                    },
                    {
                        name: 'sequenceNumbers',
                        type: {
                            vec: {
                                defined: 'MerkleTreeSequenceNumber',
                            },
                        },
                    },
                    {
                        name: 'relayFee',
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'isCompress',
                        type: 'bool',
                    },
                    {
                        name: 'compressOrDecompressLamports',
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'pubkeyArray',
                        type: {
                            vec: 'publicKey',
                        },
                    },
                    {
                        name: 'message',
                        type: {
                            option: 'bytes',
                        },
                    },
                ],
            },
        },
        {
            name: 'CompressedCpiContext',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'setContext',
                        type: 'bool',
                    },
                    {
                        name: 'cpiContextAccountIndex',
                        type: 'u8',
                    },
                ],
            },
        },
        {
            name: 'InputTokenDataWithContext',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'amount',
                        type: 'u64',
                    },
                    {
                        name: 'delegateIndex',
                        type: {
                            option: 'u8',
                        },
                    },
                    {
                        name: 'delegatedAmount',
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'isNative',
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'merkleContext',
                        type: {
                            defined: 'PackedMerkleContext',
                        },
                    },
                    {
                        name: 'rootIndex',
                        type: 'u16',
                    },
                ],
            },
        },
        {
            name: 'CompressedTokenInstructionDataTransfer',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'proof',
                        type: {
                            option: {
                                defined: 'CompressedProof',
                            },
                        },
                    },
                    {
                        name: 'mint',
                        type: 'publicKey',
                    },
                    {
                        name: 'signerIsDelegate',
                        type: 'bool',
                    },
                    {
                        name: 'inputTokenDataWithContext',
                        type: {
                            vec: {
                                defined: 'InputTokenDataWithContext',
                            },
                        },
                    },
                    {
                        name: 'outputCompressedAccounts',
                        type: {
                            vec: {
                                defined: 'PackedTokenTransferOutputData',
                            },
                        },
                    },
                    {
                        name: 'isCompress',
                        type: 'bool',
                    },
                    {
                        name: 'compressOrDecompressAmount',
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'cpiContext',
                        type: {
                            option: {
                                defined: 'CompressedCpiContext',
                            },
                        },
                    },
                ],
            },
        },
        {
            name: 'PackedTokenTransferOutputData',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'owner',
                        type: 'publicKey',
                    },
                    {
                        name: 'amount',
                        type: 'u64',
                    },
                    {
                        name: 'lamports',
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'merkleTreeIndex',
                        type: 'u8',
                    },
                ],
            },
        },
        {
            name: 'TokenTransferOutputData',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'owner',
                        type: 'publicKey',
                    },
                    {
                        name: 'amount',
                        type: 'u64',
                    },
                    {
                        name: 'lamports',
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'merkleTree',
                        type: 'publicKey',
                    },
                ],
            },
        },
        {
            name: 'TokenData',
            type: {
                kind: 'struct',
                fields: [
                    {
                        name: 'mint',
                        docs: ['The mint associated with this account'],
                        type: 'publicKey',
                    },
                    {
                        name: 'owner',
                        docs: ['The owner of this account.'],
                        type: 'publicKey',
                    },
                    {
                        name: 'amount',
                        docs: ['The amount of tokens this account holds.'],
                        type: 'u64',
                    },
                    {
                        name: 'delegate',
                        docs: [
                            'If `delegate` is `Some` then `delegated_amount` represents',
                            'the amount authorized by the delegate',
                        ],
                        type: {
                            option: 'publicKey',
                        },
                    },
                    {
                        name: 'state',
                        docs: ["The account's state"],
                        type: {
                            defined: 'AccountState',
                        },
                    },
                    {
                        name: 'isNative',
                        docs: [
                            'If is_some, this is a native token, and the value logs the rent-exempt',
                            'reserve. An Account is required to be rent-exempt, so the value is',
                            'used by the Processor to ensure that wrapped SOL accounts do not',
                            'drop below this threshold.',
                        ],
                        type: {
                            option: 'u64',
                        },
                    },
                    {
                        name: 'delegatedAmount',
                        docs: ['The amount delegated'],
                        type: 'u64',
                    },
                ],
            },
        },
        {
            name: 'AccountState',
            type: {
                kind: 'enum',
                variants: [
                    {
                        name: 'Initialized',
                    },
                    {
                        name: 'Frozen',
                    },
                ],
            },
        },
        {
            name: 'ErrorCode',
            type: {
                kind: 'enum',
                variants: [
                    {
                        name: 'PublicKeyAmountMissmatch',
                    },
                    {
                        name: 'SignerCheckFailed',
                    },
                    {
                        name: 'ComputeInputSumFailed',
                    },
                    {
                        name: 'ComputeOutputSumFailed',
                    },
                    {
                        name: 'ComputeCompressSumFailed',
                    },
                    {
                        name: 'ComputeDecompressSumFailed',
                    },
                    {
                        name: 'SumCheckFailed',
                    },
                    {
                        name: 'DecompressRecipientUndefinedForDecompress',
                    },
                    {
                        name: 'CompressedPdaUndefinedForDecompress',
                    },
                    {
                        name: 'DeCompressAmountUndefinedForDecompress',
                    },
                    {
                        name: 'CompressedPdaUndefinedForCompress',
                    },
                    {
                        name: 'DeCompressAmountUndefinedForCompress',
                    },
                    {
                        name: 'DelegateUndefined',
                    },
                    {
                        name: 'DelegateSignerCheckFailed',
                    },
                    {
                        name: 'SplTokenSupplyMismatch',
                    },
                    {
                        name: 'HeapMemoryCheckFailed',
                    },
                ],
            },
        },
    ],
    errors: [
        {
            code: 6000,
            name: 'SignerCheckFailed',
            msg: 'Signer check failed',
        },
        {
            code: 6001,
            name: 'CreateTransferInstructionFailed',
            msg: 'Create transfer instruction failed',
        },
    ],
};
