/** @type {import('next').NextConfig} */
const nextConfig = {
    output: 'export',
    images: {
        remotePatterns: [
            {
                protocol: 'https',
                hostname: 'hdslb.com',
                port: '',
                pathname: '**',
            },
        ],
    },
};

export default nextConfig;
