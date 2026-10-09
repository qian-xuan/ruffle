package {
    import flash.display.MovieClip;
    import flash.net.NetConnection;
    import flash.net.ObjectEncoding;
    import flash.net.Responder;

    public class Test extends MovieClip {
        public function Test() {
            var nc:NetConnection = new NetConnection();
            nc.objectEncoding = ObjectEncoding.AMF3;
            nc.connect("http://localhost:8000/");

            trace("--- Testing AMF3 Integer Serialization ---");
            nc.call("test.integer", new Responder(onResult, onStatus), 42);
        }

        private function onResult(result:*):void {
            trace("Received result: " + result);
        }

        private function onStatus(status:*):void {
            trace("Received status");
        }
    }
}
