'use client';
import React, { useState } from 'react';
import Link from 'next/link';
import { RxAvatar } from 'react-icons/rx';
import { payment } from '../services/api';

const Tipping: React.FC = () => {
    const [tipAmount, setTipAmount] = useState<number>(0);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setTipAmount(parseFloat(e.target.value));
    };

    const handlePayment = async () => {
        console.log(`Payment of ${tipAmount} dollars processed.`);
        try {
            const response = await payment({
                slug: '6c696e646140746573742e636f6d',
                amount_in_satoshi: tipAmount,
            });
            if (response) {
                console.log('paymentRes', response);
            }
        } catch (error) {
            console.error(error);
        }
    };

    return (
        <div className='bg-yellow-50 py-12 px-4 sm:px-6 lg:px-8 rounded'>
            <div className='max-w-md w-full space-y-8'>
                <Link href='/' className='flex items-center'>
                    {/* <Image
                        src='/lightlogo.png'
                        alt='logo'
                        width={100}
                        height={100}
                        className='mx-auto'
                    /> */}
                    <RxAvatar size={50} className='mx-auto' />
                </Link>

                <div className='bg-white rounded-lg shadow-md p-6'>
                    <h2 className='text-3xl font-extrabold text-gray-900 mb-4 text-center'>
                        Tip Creator
                    </h2>
                    <div className='flex flex-col space-y-4'>
                        <label
                            htmlFor='tipAmount'
                            className='text-gray-700 text-center'
                        >
                            Enter tip amount (SATS)
                        </label>
                        <input
                            id='tipAmount'
                            type='number'
                            min='0'
                            step='0.01'
                            value={tipAmount}
                            onChange={handleInputChange}
                            className='w-full px-3 py-2 border border-yellow-300 rounded-md focus:outline-none focus:ring-yellow-500 focus:border-yellow-500'
                        />
                    </div>
                    <button
                        onClick={handlePayment}
                        className='mt-4 w-full bg-amber-500 text-white rounded-md py-2 px-4 hover:bg-amber-400 focus:outline-none focus:ring-2 focus:ring-amber-500 focus:ring-offset-2'
                    >
                        Pay Now
                    </button>
                </div>
            </div>
        </div>
    );
};

export default Tipping;
