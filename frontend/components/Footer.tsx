import Link from 'next/link';
import { FaInstagram, FaFacebook, FaTwitter } from 'react-icons/fa';

const Footer = () => (
    <footer className='bg-gray-900 text-white py-8 mt-12 w-full'>
        <div className='container mx-auto flex justify-between items-center'>
            <div className='flex items-center gap-x-6'>
                {/* Social Icons */}
                <Link href='#' className='text-white hover:text-gray-500'>
                    <FaFacebook />
                </Link>
                <Link href='#' className='text-white hover:text-gray-500'>
                    <FaInstagram />
                </Link>
                <Link href='#' className='text-white hover:text-gray-500'>
                    <FaTwitter />
                </Link>
                {/* Add more social icons as needed */}
            </div>
            {/* Menu */}
            <nav className='flex items-center gap-x-6'>
                <Link href='#'>
                    <p className='text-gray-500 hover:text-white'>FAQ</p>
                </Link>
                <Link href='#'>
                    <p className='text-gray-500 hover:text-white'>Terms</p>
                </Link>
                <Link href='#'>
                    <p className='text-gray-500 hover:text-white'>Privacy</p>
                </Link>
            </nav>
        </div>
        {/* Copyright */}
        <div className='text-center mt-4'>
            <p className='text-sm'>
                &copy; {new Date().getFullYear()} TipJar. All Rights Reserved.
            </p>
        </div>
    </footer>
);

export default Footer;
