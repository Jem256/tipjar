'use client';
import Link from 'next/link';
import NavLink from '../components/NavLink';

export default function Home() {
    return (
        <section>
            <div className='text-gray-600'>
                <div className='space-y-5 max-w-4xl mx-auto text-center'>
                    <h1 className='text-4xl text-gray-800 font-extrabold mx-auto sm:text-6xl'>
                        Generate your next payment Link in seconds
                    </h1>
                    <p className='max-w-xl mx-auto'>
                        TipJar makes it simple for you to generate lightning
                        payment links for your audience.
                    </p>
                    <div className='flex items-center justify-center gap-x-3 font-medium text-sm'>
                        <Link
                            href='/register'
                            className='text-white bg-gray-800 hover:bg-gray-600 active:bg-gray-900 py-2.5 px-4 text-center rounded-lg duration-150 '
                        >
                            Generate Your Link
                        </Link>
                        <NavLink
                            target='_blank'
                            href='#'
                            className='text-gray-700 border hover:bg-gray-50'
                            scroll={false}
                        >
                            Learn more
                        </NavLink>
                    </div>
                </div>
            </div>
        </section>
    );
}
