import http from 'k6/http';
import { sleep } from 'k6';
export const options = {
  vus: __ENV.VUS,
  duration: __ENV.DURATION,
};
export default function () {
  http.get(`http://${__ENV.CLUSTER_EXTERNAL_IP}:30000${__ENV.API}`);
  sleep(__ENV.K6_SLEEP);
}