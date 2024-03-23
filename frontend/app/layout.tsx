import Navbar from '@/components/Navbar';
import './globals.css';
import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import Footer from '@/components/Footer';
import { Analytics } from '@vercel/analytics/react';
import PlausibleProvider from 'next-plausible';

const inter = Inter({ subsets: ['latin'] });

let title = 'TipJar - Lightning-Powered Tipping Platform';
let description = 'Generate your Payment Link in seconds';
let url = '/';
let ogimage = '/';
let sitename = '/';

export const metadata: Metadata = {
    //   metadataBase: new URL(url),
    title,
    description,
    icons: {
        icon: '/favicon.ico',
    },
    openGraph: {
        // images: [ogimage],
        title,
        description,
        url: url,
        siteName: sitename,
        locale: 'en_US',
        type: 'website',
    },
    twitter: {
        card: 'summary_large_image',
        // images: [ogimage],
        title,
        description,
    },
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang='en'>
            <head>
                <PlausibleProvider domain='/' />
            </head>
            <body className={`flex flex-col min-h-screen ${inter.className}`}>
                <Navbar />
                <main className='flex-1 flex justify-center items-center'>
                    {children}
                </main>
                <Analytics />
                <Footer />
            </body>
        </html>
    );
}
