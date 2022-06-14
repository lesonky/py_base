from . import BaseTest


class TestAPIExample(BaseTest):

    def test_ping_api(self, client):
        resp = client.get('/api/ping')
        assert resp.status_code == 200
