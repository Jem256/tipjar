'use client';
import React, { useEffect, useState } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import axios from 'axios';
import { useSelector } from 'react-redux';
import { RootState } from '../services/rootReducer';
import { FaCopy } from 'react-icons/fa';

const Dashboard = () => {
    const user = useSelector((state: RootState) => state.auth.user);

    const userSlug = `http://localhost:3000/${user?.slug}`;

    const handleCopy = () => {
        navigator.clipboard
            .writeText(userSlug)
            .then(() => {
                console.log('Text copied to clipboard:', userSlug);
            })
            .catch((error) => {
                console.error('Error copying text:', error);
            });
    };

    return (
        <div className='bg-gray-50 py-4 px-4 sm:px-6 lg:px-8 rounded shadow-lg'>
            <div className='max-w-md w-full space-y-8'>
                <Link href='/' className='flex items-center'>
                    <Image
                        src='/lightlogo.png'
                        alt='logo'
                        width={100}
                        height={100}
                        className='mx-auto'
                    />
                </Link>

                <div className='text-center'>
                    <h2 className='mt-6 text-3xl font-extrabold text-gray-900'>
                        Welcome to Your Dashboard
                    </h2>
                    <p className='mt-2 text-sm text-gray-600'>
                        Here you can view your profile, account balance, and
                        share payment links.
                    </p>
                </div>

                <div className='mt-8 space-y-6'>
                    <div className='flex flex-col items-center'>
                        <h3 className='text-lg font-semibold'>User Profile</h3>
                        {/* Display user profile information */}
                        <p className='text-gray-600'>Name: {user?.name}</p>
                        <p className='text-gray-600'>Email: {user?.email}</p>
                    </div>

                    <div className='flex flex-col items-center'>
                        <h3 className='text-lg font-semibold'>
                            Account Balance
                        </h3>
                        {/* Display user account balance */}
                        <p className='text-gray-600'>{user?.balance}</p>
                    </div>

                    <div className='flex flex-col items-center'>
                        <h3 className='text-lg font-semibold'>
                            Sharable Payment Link
                        </h3>
                        {/* Display sharable payment link */}
                        <label htmlFor='shareable-link' className='sr-only'>
                            Sharable Payment Link
                        </label>

                        <div className='flex'>
                            <input
                                type='text'
                                className='w-full px-3 py-2 mt-1 text-gray-900 rounded-md border border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500'
                                value={userSlug}
                                placeholder='Sharable Payment Link'
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
            </div>
        </div>
    );
};

export default Dashboard;
