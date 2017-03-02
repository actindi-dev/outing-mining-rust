$(function() {
    var ctx = document.getElementById("chart");
    var chartData = $(ctx).data('chart-data');
    var labels = $.map(chartData, function(x) { return x.date; });
    var failedIpCounts = $.map(chartData, function(x) {
        var n = 0;
        $.each(x.failed, function() { ++n });
        return n;
    });
    var failedCounts = $.map(chartData, function(x) {
        var n = 0;
        $.map(x.failed, function(v) { n += v });
        return n;
    });
    var oauthFailedCounts = $.map(chartData, function(x) { return x.oauth_failed });
    var passwordResetRequestFailedCounts = $.map(chartData, function(x) {
        return x.password_reset_request_failed;
    });
    var passwordResetFailedCounts = $.map(chartData, function(x) {
        return x.password_reset_failed;
    });

    var myChart = new Chart(ctx, {
        //type: 'bar',
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                { label: 'ログイン失敗IP数',
                  data: failedIpCounts,
                  lineTension: 0,
                  fill: false,
                  borderColor: "rgba(75,192,192,1)",
                },
                { label: 'ログイン失敗数',
                  data: failedCounts,
                  lineTension: 0,
                  fill: false,
                  borderColor: "rgba(192,75,192,1)",
                },
                { label: 'OAuth失敗数',
                  data: oauthFailedCounts,
                  lineTension: 0,
                  fill: false,
                  borderColor: "rgba(192,192,75,1)",
                },
                { label: 'パスワードリセットリクエスト失敗数',
                  data: passwordResetRequestFailedCounts,
                  lineTension: 0,
                  fill: false,
                  borderColor: "rgba(75,75,192,1)",
                },
                { label: 'パスワードリセットリ失敗数',
                  data: passwordResetFailedCounts,
                  lineTension: 0,
                  fill: false,
                  borderColor: "rgba(75,192,75,1)",
                }
            ]
        },
        options: {
            scales: {
                yAxes: [{
                    ticks: {
                        beginAtZero:true
                    }
                }]
            }
        }
    });
});
