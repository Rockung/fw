package top.lesiji.learning.redis;

import junit.framework.Test;
import junit.framework.TestCase;
import junit.framework.TestSuite;

import redis.clients.jedis.Jedis;
import redis.clients.jedis.JedisPool;
import redis.clients.jedis.JedisPoolConfig;

/**
 * Unit test for simple App.
 */
public class AppTest 
    extends TestCase
{
    /**
     * Create the test case
     *
     * @param testName name of the test case
     */
    public AppTest( String testName )
    {
        super( testName );
    }

    /**
     * @return the suite of tests being tested
     */
    public static Test suite()
    {
        return new TestSuite( AppTest.class );
    }

    /**
     * Rigourous Test :-)
     */
    public void testApp()
    {
        assertTrue( true );
    }
	
	public void testRedis() {
		JedisPool pool = new JedisPool(new JedisPoolConfig(), "localhost");
		Jedis jedis = pool.getResource();
	
		try {	
			jedis.set("MSG", "Hello World");			
			assertEquals("Hello World", jedis.get("MSG"));			
		} finally {
			jedis.close();
		}
	}
}
