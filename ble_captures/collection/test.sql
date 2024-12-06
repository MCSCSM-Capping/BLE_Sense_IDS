SELECT client_packet.advertising_address, client_packet.device_id FROM client_packet WHERE client_packet.device_id like '6';
ELECT DISTINCT advertising_address, device_id
FROM client_packet
WHERE device_id IN (
    SELECT device_id
    FROM client_packet
    GROUP BY device_id
    HAVING COUNT(DISTINCT advertising_address) > 1
);
-- 2024-12-05 00:19:13.658485   
-- 2024-12-05 00:19:12.946538   
SELECT MAX(client_packet.time_stamp) FROM client_packet WHERE client_packet.advertising_address like '209005070789164';
SELECT MAX(client_packet.time_stamp) FROM client_packet WHERE client_packet.advertising_address like '129291415251651';
SELECT * FROM client_device;

SELECT DISTINCT "client_device"."id", COUNT("client_packet"."id")
FILTER 
    (WHERE "client_packet"."time_stamp" BETWEEN 2024-11-29 00:00:00 AND 2024-12-06 23:59:59.999999) AS "total_packets", 
    COUNT("client_packet"."id") 
    FILTER (WHERE ("client_packet"."time_stamp" BETWEEN 2024-11-29 00:00:00 AND 2024-12-06 23:59:59.999999 AND "client_packet"."malicious"))
        AS "malicious_packets",
    CASE WHEN COUNT("client_packet"."id")
        FILTER (WHERE ("client_packet"."time_stamp" BETWEEN 2024-11-29 00:00:00 AND 2024-12-06 23:59:59.999999)) > 0
        THEN ((COUNT("client_packet"."id") 
                FILTER (WHERE ("client_packet"."time_stamp" BETWEEN 2024-11-29 00:00:00 AND 2024-12-06 23:59:59.999999 AND "client_packet"."malicious")) * 100.0) / COUNT("client_packet"."id") 
            FILTER (WHERE "client_packet"."time_stamp" BETWEEN 2024-11-29 00:00:00 AND 2024-12-06 23:59:59.999999)) 
        ELSE 0.0 END AS "malicious_percentage" FROM "client_device" LEFT OUTER JOIN "client_packet" ON ("client_device"."id" = "client_packet"."device_id") 
            WHERE "client_packet"."time_stamp" BETWEEN 2024-11-29 00:00:00 AND 2024-12-06 23:59:59.999999 GROUP BY "client_device"."id"
