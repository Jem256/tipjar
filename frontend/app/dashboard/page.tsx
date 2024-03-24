// pages/dashboard.tsx

import React from 'react';
import Link from 'next/link';
import Image from 'next/image';

const Dashboard: React.FC = () => {
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

                {/* <div className='text-center'>
                    <h2 className='mt-6 text-3xl font-extrabold text-gray-900'>
                        Welcome to Your Dashboard
                    </h2>
                    <p className='mt-2 text-sm text-gray-600'>
                        Here you can view your profile, account balance, and
                        share payment links.
                    </p>
                </div> */}

                <div className='mt-8 space-y-6'>
                    <div className='flex flex-col items-center'>
                        <h3 className='text-lg font-semibold'>User Profile</h3>
                        {/* Display user profile information */}
                        {/* You can replace this with actual user data */}
                        <p className='text-gray-600'>Name: John Doe</p>
                        <p className='text-gray-600'>Email: john@example.com</p>
                        {/* Add more profile information as needed */}
                    </div>

                    <div className='flex flex-col items-center'>
                        <h3 className='text-lg font-semibold'>
                            Account Balance
                        </h3>
                        {/* Display user account balance */}
                        {/* You can replace this with actual account balance data */}
                        <p className='text-gray-600'>$5000.00</p>
                    </div>

                    <div className='flex flex-col items-center'>
                        <h3 className='text-lg font-semibold'>
                            Sharable Payment Link
                        </h3>
                        {/* Display sharable payment link */}
                        {/* You can replace this with an actual sharable payment link */}
                        <label htmlFor='shareable-link' className='sr-only'>
                            Sharable Payment Link
                        </label>
                        <input
                            type='text'
                            className='w-full px-3 py-2 mt-1 text-gray-900 rounded-md border border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500'
                            value='https://example.com/payment-link'
                            placeholder='Sharable Payment Link'
                            readOnly
                        />
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Dashboard;
