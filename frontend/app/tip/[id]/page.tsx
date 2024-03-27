'use client';
import React, { useState } from 'react';
import Link from 'next/link';
import { RxAvatar } from 'react-icons/rx';
import { payment } from '../../services/api';
import QRCode from 'react-qr-code';
import { FaCopy } from 'react-icons/fa';
import { usePathname } from 'next/navigation';

const Tipping = ({ params }: { params: { slug: string } }) => {
    const pathname = usePathname();
    const id = pathname.split('/').pop();

    const [tipAmount, setTipAmount] = useState<number>(0);
    const [paid, setPaid] = useState(false);
    const [qrValue, setQrValue] = useState('');

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setTipAmount(parseFloat(e.target.value));
    };

    const handlePayment = async () => {
        const userData = {
            slug: id,
            amount_in_satoshi: tipAmount,
        };

        try {
            const response = await payment(userData);
            if (response) {
                console.log('paymentRes', response);
                setPaid(true);
                setQrValue(response.payment_request);
            }
        } catch (error) {
            console.error('paymentErr', error);
            setPaid(false);
        }
    };

    const handleCopy = () => {
        navigator.clipboard
            .writeText(qrValue)
            .then(() => {
                console.log('Text copied to clipboard:', qrValue);
            })
            .catch((error) => {
                console.error('Error copying text:', error);
            });
    };

    return (
        <>
            {!paid ? (
                <div className='bg-yellow-50 py-12 px-4 sm:px-6 lg:px-8 rounded'>
                    <div className='max-w-md w-full space-y-8'>
                        <Link href='/' className='flex items-center'>
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
            ) : (
                <div className='bg-yellow-50 py-4 px-4 sm:px-6 lg:px-8 rounded'>
                    <div
                        style={{
                            height: 'auto',
                            margin: '0 auto',
                            width: '100%',
                        }}
                        className='flex flex-col gap-3'
                    >
                        <QRCode
                            size={256}
                            style={{
                                height: 'auto',
                                maxWidth: '100%',
                                width: '100%',
                            }}
                            value={qrValue}
                            viewBox={`0 0 256 256`}
                        />
                        <label
                            htmlFor='payment-reference'
                            className='text-xl text-gray-800 font-extrabold mx-auto sm:text-xl'
                        >
                            Payment Reference
                        </label>
                        <div className='flex'>
                            <input
                                id='payment-reference'
                                name='payment-reference'
                                type='text'
                                className='w-full px-3 py-2 mt-1 text-gray-900 rounded-md border border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500'
                                value={qrValue}
                                placeholder='Payment Reference'
                                readOnly
                            />
                            <button
                                title='Copy Payment Reference'
                                onClick={handleCopy}
                                className='flex-shrink-0 px-3 py-2 mt-1 ml-1 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500'
                            >
                                <FaCopy />
                            </button>
                        </div>
                    </div>
                </div>
            )}
        </>
    );
};

export default Tipping;
