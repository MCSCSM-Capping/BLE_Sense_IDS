SELECT client_packet.advertising_address, client_packet.device_id FROM client_packet WHERE client_packet.device_id like '6';
ELECT DISTINCT advertising_address, device_id
FROM client_packet
WHERE device_id IN (
    SELECT device_id
    FROM client_packet
    GROUP BY device_id
    HAVING COUNT(DISTINCT advertising_address) > 1
);
SELECT MAX(client_packet.time_stamp) FROM client_packet WHERE client_packet.advertising_address like '209005070789164';
SELECT MAX(client_packet.time_stamp) FROM client_packet WHERE client_packet.advertising_address like '129291415251651';
SELECT * FROM client_device;
