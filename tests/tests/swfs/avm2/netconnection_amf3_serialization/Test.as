package {
    import flash.display.MovieClip;
    import flash.net.NetConnection;
    import flash.net.ObjectEncoding;
    import flash.net.Responder;

    public class Test extends MovieClip {
        public function Test() {
            var nc:NetConnection = new NetConnection();

            // Set AMF3 encoding — the request packet should use version byte 0x03
            // instead of 0x00 (AMF0).
            nc.objectEncoding = ObjectEncoding.AMF3;
            nc.connect("http://localhost:8000/");

            trace("--- Testing AMF3 NetConnection Serialization ---");
            nc.call("test.echo", new Responder(onResult, onStatus), "hello");
        }

        private function onResult(result:*):void {
            trace("Received result: " + result);
        }

        private function onStatus(status:*):void {
            trace("Received status");
        }
    }
}
