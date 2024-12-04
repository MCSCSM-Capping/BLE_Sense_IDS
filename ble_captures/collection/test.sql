SELECT client_packet.advertising_address, client_packet.device_id FROM client_packet WHERE client_packet.device_id like '6';
SELECT DISTINCT advertising_address, device_id
FROM client_packet
WHERE device_id IN (
    SELECT device_id
    FROM client_packet
    GROUP BY device_id
    HAVING COUNT(DISTINCT advertising_address) > 1
);
SELECT MAX(client_packet.time_stamp) FROM client_packet WHERE client_packet.device_id like '10';
SELECT * FROM client_device;
