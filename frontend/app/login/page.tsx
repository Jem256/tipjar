// pages/login.tsx

import React from 'react';
import { FaLock } from 'react-icons/fa';

const Login: React.FC = () => {
    return (
        <div className='px-4 sm:px-6 lg:px-8'>
            <div className='max-w-md w-full space-y-8'>
                <div>
                    <h2 className='mt-4 text-center text-3xl font-extrabold text-gray-900'>
                        Sign in to your account
                    </h2>
                </div>
                <form className='mt-8 space-y-6' action='#' method='POST'>
                    <input type='hidden' name='remember' value='true' />
                    <div className='rounded-md shadow-sm space-y-6'>
                        <div>
                            <label htmlFor='email-address' className='sr-only'>
                                Email address
                            </label>
                            <input
                                id='email-address'
                                name='email'
                                type='email'
                                autoComplete='email'
                                required
                                className='appearance-none rounded-md relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm'
                                placeholder='Email address'
                            />
                        </div>
                        <div>
                            <label htmlFor='password' className='sr-only'>
                                Password
                            </label>
                            <input
                                id='password'
                                name='password'
                                type='password'
                                autoComplete='current-password'
                                required
                                className='appearance-none rounded-md relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm'
                                placeholder='Password'
                            />
                        </div>
                    </div>

                    <div className='flex items-center justify-between'>
                        <div className='flex items-center'>
                            <input
                                id='remember-me'
                                name='remember-me'
                                type='checkbox'
                                className='h-4 w-4 text-gray-600 focus:ring-gray-500 border-gray-300 rounded'
                            />
                            <label
                                htmlFor='remember-me'
                                className='ml-2 block text-sm text-gray-900'
                            >
                                Remember me
                            </label>
                        </div>

                        <div className='text-sm'>
                            <a
                                href='#'
                                className='font-medium text-gray-600 hover:text-gray-500'
                            >
                                Forgot your password?
                            </a>
                        </div>
                    </div>

                    <div>
                        <button
                            type='submit'
                            className='group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-gray-800 hover:bg-gray-600 active:bg-gray-900 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:bg-gray-500'
                        >
                            <span className='absolute left-0 inset-y-0 flex items-center pl-3'>
                                {/* <!-- Heroicon name: solid/lock-closed --> */}
                                <FaLock />
                            </span>
                            Sign in
                        </button>
                    </div>
                </form>
            </div>
        </div>
    );
};

export default Login;
