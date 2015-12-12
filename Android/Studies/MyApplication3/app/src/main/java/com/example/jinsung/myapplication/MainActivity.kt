package com.example.jinsung.myapplication


import android.content.Context
import android.graphics.Color
import android.net.ConnectivityManager
import android.net.NetworkInfo
import android.os.AsyncTask
import android.os.Bundle
import android.support.design.widget.FloatingActionButton
import android.support.design.widget.Snackbar
import android.support.v7.app.AppCompatActivity
import android.support.v7.widget.Toolbar
import android.util.Log
import android.view.View
import android.view.Menu
import android.view.MenuItem
import android.widget.LinearLayout
import android.widget.TextView

import java.io.IOException
import java.io.InputStream
import java.io.InputStreamReader
import java.io.Reader
import java.net.HttpURLConnection
import java.net.MalformedURLException
import java.net.NetworkInterface
import java.net.URL
import java.nio.CharBuffer

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        val toolbar = findViewById(R.id.toolbar) as Toolbar
        setSupportActionBar(toolbar)

        val fab = findViewById(R.id.fab) as FloatingActionButton
        fab.setOnClickListener { view -> Snackbar.make(view, "Replace with your own action", Snackbar.LENGTH_LONG).setAction("Action", null).show() }
    }

    override fun onCreateOptionsMenu(menu: Menu): Boolean {
        // Inflate the menu; this adds items to the action bar if it is present.
        menuInflater.inflate(R.menu.menu_main, menu)
        return true
    }

    public override fun onResume() {
        super.onResume()

        NetworkTask().execute("http://www.naver.com/", "")
    }

    override fun onOptionsItemSelected(item: MenuItem): Boolean {
        // Handle action bar item clicks here. The action bar will
        // automatically handle clicks on the Home/Up button, so long
        // as you specify a parent activity in AndroidManifest.xml.
        val id = item.itemId

        //noinspection SimplifiableIfStatement
        if (id == R.id.action_settings) {
            return true
        }

        return super.onOptionsItemSelected(item)
    }


    private inner class NetworkTask : AsyncTask<String, Int, Long>() {
        internal var response: String? = null

        override fun doInBackground(vararg urlstr: String): Long? {
            val connMgr = getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
            val netInfo = connMgr.activeNetworkInfo
            if (netInfo == null || netInfo.isConnected == false) {
                val tv = TextView(applicationContext)
                tv.text = "network inactive"
                tv.setTextColor(Color.BLUE)
                val layout = findViewById(R.id.linearLayout) as LinearLayout
                layout.addView(tv)
            } else {
                try {
                    val url = URL(urlstr[0])
                    val conn = url.openConnection() as HttpURLConnection
                    conn.connect()
                    val response = conn.responseCode
                    Log.d("aaa", response.toString())
                    val `is` = conn.inputStream
                    val reader = InputStreamReader(`is`, "UTF-8")
                    val cb: CharBuffer? = null
                    reader.read(cb)
                    Log.d("bbb", cb!!.toString())
                    val tv = TextView(applicationContext)
                    tv.text = cb.toString()
                    tv.setTextColor(Color.BLUE)
                    val layout = findViewById(R.id.linearLayout) as LinearLayout
                    layout.addView(tv)
                } catch (e: MalformedURLException) {
                    e.printStackTrace()
                } catch (e: IOException) {
                    e.printStackTrace()
                }

            }

            return 0
        }

        protected override fun onProgressUpdate(vararg progress: Int) {

        }

        override fun onPostExecute(result: Long?) {

        }

        fun execute(s: String, s1: String) {
        }
    }
}
