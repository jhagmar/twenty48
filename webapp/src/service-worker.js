import {precacheAndRoute} from 'workbox-precaching';
import { registerRoute } from 'workbox-routing';
import {
    StaleWhileRevalidate,
    CacheFirst,
} from 'workbox-strategies';

import { CacheableResponsePlugin } from 'workbox-cacheable-response';
import { ExpirationPlugin } from 'workbox-expiration';

precacheAndRoute(self.__WB_MANIFEST);
